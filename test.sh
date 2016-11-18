#!/usr/bin/env bash

EXIT_STATUS=0
(cd lalrpop-util && cargo test) || EXIT_STATUS=$?
(cd lalrpop-intern && cargo test) || EXIT_STATUS=$?
(cd lalrpop && cargo test) || EXIT_STATUS=$?
(cd lalrpop-test && cargo test) || EXIT_STATUS=$?
(cd doc/calculator && cargo test) || EXIT_STATUS=$?
(cd doc/whitespace && cargo test) || EXIT_STATUS=$?
(cd doc/pascal/lalrpop && cargo test) || EXIT_STATUS=$?
exit $EXIT_STATUS
