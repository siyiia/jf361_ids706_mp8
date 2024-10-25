p_install:
	pip install -r requirements.txt
p_lint:
	pylint --disable=R,C *.py
p_format:
	black *.py
p_test:
	python -m pytest -vv test_*.py
p_run:
	python main.py


r_format:
	cargo fmt --quiet
r_lint:
	cargo clippy --quiet
r_test:
	cargo test --quiet
r_run:
	cargo run