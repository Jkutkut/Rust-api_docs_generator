OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

docker:
	docker run -it -v $(CURRENT_PATH):/app -w /app jkutkut/docker4rust