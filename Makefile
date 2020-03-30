all:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests; done

test:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests run; done

clean:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests clean; done
