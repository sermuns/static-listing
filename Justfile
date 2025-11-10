release:
	cargo release --execute $(git cliff --bumped-version | cut -d'v' -f2)

watch:
	cargo watch -i public -x run

serve:
	~/.cargo/bin/http-server -i public

[parallel]
dev: serve watch
