PROGRAM a1 (input,output);
    CONST 
	NotScheduled = '        ';

	EmployeeMaxLen = 8;

	
	FirstHour = 8;
	LastHour = 17;		
	PastLastHour = 18;	

	
	TableDayWidth = 9;
    TYPE 
	
	EmployeeType = ARRAY [EmployeeMaxLen] OF string;

	
	
	HourType = 8..17;
	ScheduleType = ARRAY [HourType, DayType] OF EmployeeType;
	
	HourScanType = 8..18;

    VAR
	
	Schedule: ScheduleType;

	
	KeepRunning: boolean;

        
        Command: string;

    
    PROCEDURE ReadString(VAR Str: string);
	VAR
	    Ch: char;
	BEGIN
	    Ch := ' ';
	    WHILE (Ch = ' ') AND NOT eoln DO 
		read(Ch);

	    IF Ch = ' ' THEN
		
		Str := ''
	    ELSE
		BEGIN 
		    
		    Str := '';
		    WHILE (Ch <> ' ') AND NOT eoln DO
			BEGIN
			    Str := Str + Ch;
			    read(Ch)
			END;

		    IF Ch <> ' ' THEN
			
			Str := Str + Ch
		END
	END; 

    
    PROCEDURE ReadSchedClrArgs(
	    VAR StartDay, EndDay: DayType;	
	    VAR StartHour, EndHour: HourType;	
	    VAR Error: boolean);		
	VAR
	    InputHour: integer;			

	
	FUNCTION MapTo24(Hour: integer): HourType;
	    CONST
		
		LastPM = 5;
	    BEGIN
		IF Hour <= LastPM THEN
		    MapTo24 := Hour + 12
		ELSE
		    MapTo24 := Hour
	    END;

	BEGIN 
	    
	    ReadDay(input, StartDay);
	    ReadDay(input, EndDay);

	    
	    IF (StartDay <> BadDay) AND (EndDay <> BadDay) THEN 
		BEGIN
		    
		    read(InputHour);
		    StartHour := MapTo24(InputHour);
		    read(InputHour);
		    EndHour := MapTo24(InputHour);

		    
		    Error := FALSE 
		END
	    ELSE
		
		Error := TRUE;

	    
	    readln
	END; 

    
    PROCEDURE WriteDaysHeader;
	CONST

	    
	    DaysHeadMoveOver = 6;

	    
	    AllowForDay = 3;
	VAR
	    Day: DayType;
	BEGIN
	    write(' ': DaysHeadMoveOver);

	    FOR Day := Sun TO Sat DO
		BEGIN
		    write('[ ');
		    WriteDay(output, Day);
		    write(' ]', ' ': TableDayWidth - AllowForDay - 4)
		END;
	    writeln
	END; 

    
    FUNCTION SchedLegal(
	    VAR Schedule: ScheduleType;	    
		StartDay, EndDay: DayType;  
		FirstHour, LastHour: 	    
			HourType): boolean;
	VAR
	    ConflictFound: boolean;	    
	    DayScan: DayType;		    
	    HourScan: HourScanType;	    
	BEGIN
	    
	    DayScan := StartDay;
	    ConflictFound := FALSE;
	    REPEAT
		
		HourScan := FirstHour;
		WHILE NOT ConflictFound AND
				(HourScan <= LastHour) DO BEGIN
		    
		    ConflictFound :=
			    Schedule[HourScan, DayScan] <> NotScheduled;

		    
		    HourScan := HourScan + 1
		END;

		
		DayScan := succ(DayScan)
	    UNTIL ConflictFound OR (DayScan > EndDay);

	    
	    SchedLegal := NOT ConflictFound
	END; 

    
    PROCEDURE SetSchedPart(
	    VAR Schedule: ScheduleType;	    
		Employee: EmployeeType;	    
		StartDay, EndDay: DayType;  
		FirstHour, LastHour:	    
				HourType);
	VAR
	    DayScan: DayType;		    
	    HourScan: HourType;		    
	BEGIN
	    FOR DayScan := StartDay TO EndDay DO
		FOR HourScan := FirstHour TO LastHour DO
		    Schedule[HourScan, DayScan] := Employee
	END; 

    
    PROCEDURE DoSched(
	    VAR Schedule: ScheduleType);    
	VAR
	    Employee: EmployeeType;	    
	    StartDay, EndDay: DayType;	    
	    StartHour, EndHour: HourType;   
	    Error: boolean;		    
	BEGIN
	    
	    ReadString(Employee);

	    
	    ReadSchedClrArgs(StartDay, EndDay, StartHour, EndHour, Error);

	    
	    IF Error THEN
		writeln('*** Un-recognized day code.  ',
		    'Command not performed. ***')
	    ELSE 
		
		IF SchedLegal(Schedule, StartDay, EndDay,
					StartHour, EndHour) THEN
		    BEGIN
			
			SetSchedPart(Schedule, Employee,
				StartDay, EndDay, StartHour, EndHour);
			writeln('>>> ', Employee, ' scheduled. <<<')
		    END
		ELSE 
		    
		    writeln('*** Conflicts with existing schedule.  ',
			'Command not performed. ***')
	END; 

    
    PROCEDURE DoClear(
	    VAR Schedule: ScheduleType);    
	VAR
	    StartDay, EndDay: DayType;	    
	    StartHour, EndHour: HourType;   
	    Error: boolean;		    
	BEGIN
	    
	    ReadSchedClrArgs(StartDay, EndDay, StartHour, EndHour, Error);

	    
	    IF Error THEN
		writeln('*** Un-recognized day code.  ',
		    'Command not performed. ***')
	    ELSE 
		BEGIN
		    SetSchedPart(Schedule, NotScheduled, StartDay, EndDay,
			StartHour, EndHour);
		    writeln('>>> Clear performed. <<<');
		END 
	END;

    
    PROCEDURE DoUnsched(
	    VAR Schedule: ScheduleType);	
	VAR
	    Employee: EmployeeType;		
	    Day: DayType;			
	    Hour: integer;			
	    Found: boolean;			
	BEGIN
	    
	    readln(Employee);

	    
	    Found := FALSE;
	    FOR Day := Sun TO Sat DO
		FOR Hour := FirstHour TO LastHour DO
		    IF Schedule[Hour, Day] = Employee THEN 
			BEGIN
			    
			    Schedule[Hour, Day] := NotScheduled;

			    
			    Found := TRUE 
			END;

	    
	    IF Found THEN 
		write('>>> ', Employee, ' removed from schedule. <<<')
	    ELSE
		write('>>> ', Employee, 
				    ' was not on the schedule. <<<')
	END; 

    
    PROCEDURE DoPrint(
	    VAR Schedule: ScheduleType);	
	VAR
	    Hour: HourType;			
	    Day: DayType;			

	
	FUNCTION Map24to12(HourType: HourType): integer;
	    BEGIN
		IF Hour < 13 THEN
		    Map24to12 := Hour
		ELSE
		    Map24to12 := Hour - 12
	    END;
	BEGIN
	    readln;
	    WriteDaysHeader;

	    FOR Hour := FirstHour TO LastHour DO
		BEGIN
		    write(Map24to12(Hour):2, ':00 ');
		    FOR Day := Sun TO Sat DO
			write(Schedule[Hour, Day], 
			    ' ': TableDayWidth - length(Schedule[Hour, Day]));
		    writeln
		END
	END;

    
    PROCEDURE DoTotal(
	    VAR Schedule: ScheduleType);	
	VAR
	    Employee: EmployeeType;		
	    Day: DayType;			
	    Hour: integer;			
	    Total: integer;			
	BEGIN
	    
	    readln(Employee);

	    
	    Total := 0;
	    FOR Day := Sun TO Sat DO
		FOR Hour := FirstHour TO LastHour DO
		    IF Schedule[Hour, Day] = Employee THEN
			Total := Total + 1;

	    
	    writeln('>>> ', Employee,
		' is scheduled for ', Total:1, ' hours. <<<<')
	END; 

    

    BEGIN
        
        SetSchedPart(Schedule, NotScheduled, Sun, Sat, FirstHour, LastHour);
 
        
	write('==> ');
    	ReadString(Command);
	KeepRunning := TRUE;
        WHILE KeepRunning DO
	    BEGIN
		IF Command = 'sched' THEN 
            	    DoSched(Schedule)
		ELSE IF Command = 'clear' THEN
		    DoClear(Schedule)
		ELSE IF Command = 'unsched' THEN
		    DoUnsched(Schedule)
		ELSE IF Command = 'print' THEN
         	    DoPrint(Schedule)
		ELSE IF Command = 'total' THEN
		    DoTotal(Schedule)
		ELSE IF Command = 'quit' THEN 
		    BEGIN
			writeln;
			writeln('>>> Program terminating. <<<');
			KeepRunning := FALSE
		    END
		ELSE
		    
		    BEGIN
			readln;
			writeln;
			writeln('*** Command ', Command, 
						    ' not recognized. ***');
		    END;

		
		write('==> ');
		ReadString(Command)
	    END
    END.
