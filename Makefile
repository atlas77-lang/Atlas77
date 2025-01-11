update-changelog:
	@git cliff --unreleased --tag="$(TAG)" --prepend CHANGELOG.md