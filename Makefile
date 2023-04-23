test:
		sh ./test.sh

clean:
		rm -rf tmp/*.o

.PHONY: test clean