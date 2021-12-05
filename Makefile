help:
	@echo "Default help documentation"

build:
	(cd todo-cli && cargo check)

format:
	(cd todo-cli && cargo fmt)