RLIB = target/debug/libyajl.so
CFLAGS = -Ibuild/yajl-2.1.1/include
YAJL_TEST = build/test/parsing/yajl_test
YAJL_TEST_RS = target/debug/yajl_test

bin/parse_config: examples/parse_config.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

bin/json_verify: verify/json_verify.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

$(YAJL_TEST): tests/parsing/yajl_test.c $(RLIB)
	$(CC) -Wall $(CFLAGS) tests/parsing/yajl_test.c -l:libyajl.so -Ltarget/debug -o $@

$(YAJL_TEST_RS): examples/yajl_test/src/main.rs examples/yajl_test/Cargo.toml
	cargo build -p yajl_test

$(RLIB): crates/yajl-clib/Cargo.toml crates/yajl-clib/src/*.rs
	cargo build

run-parse-config: bin/parse_config
	LD_LIBRARY_PATH=target/debug bin/parse_config < examples/sample.config

run-json-verify: bin/json_verify
	LD_LIBRARY_PATH=target/debug bin/json_verify -c < examples/sample.config

test-parsing: $(YAJL_TEST)
	# cd test/parsing && ./run_tests.sh
	cd tests/parsing && LD_LIBRARY_PATH=../../target/debug ./run_tests.sh

test-parsing-rs: $(YAJL_TEST_RS)
	cd tests/parsing && ./run_tests.sh "../../$(YAJL_TEST_RS)"

bin/perftest: perf/documents.o perf/perftest.o $(RLIB)
	$(CC) -Wall $(CFLAGS) -o $@ $^

run-perftest: bin/perftest
	LD_LIBRARY_PATH=target/debug bin/perftest

target/debug/perftest: examples/perftest/*.rs
	cargo build

.PHONY: run-perftest-rs
run-perftest-rs: target/debug/perftest
	target/debug/perftest
