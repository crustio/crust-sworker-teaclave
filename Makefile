.PHONY: build clean

build:
	@$(MAKE) -C src all --no-print-directory

clean:
	@$(MAKE) -C src clean --no-print-directory
