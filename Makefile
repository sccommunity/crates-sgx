all:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests; done

test:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests run; done

clean:
	@for i in $$(ls crates); do make -C crates/$$i/sgx-tests clean; done
	@cargo clean --manifest-path example/Cargo.toml

doc:
	@cargo doc --manifest-path example/Cargo.toml
	@echo "<meta http-equiv=refresh content=0;url=`echo crates_sgx_example | cut -d '/' -f 2`/index.html>" > example/target/doc/index.html

ghp: doc
	@ghp-import -n example/target/doc
	@git push -fq git@github.com:universal-secure-computing-community/crates-sgx.git gh-pages
