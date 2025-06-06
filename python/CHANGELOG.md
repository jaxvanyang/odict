# Changelog

## [1.6.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.5.0...python/v1.6.0) (2025-05-10)


### Features

* Add `rank` attribute to Entry for storing word frequencies ([#1210](https://github.com/TheOpenDictionary/odict/issues/1210)) ([2b2439a](https://github.com/TheOpenDictionary/odict/commit/2b2439a4dcb82d2b2c247174eb22d4a90d2037d5))

## [1.5.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.4.0...python/v1.5.0) (2025-04-24)


### Features

* Add &lt;pronunciation&gt; tag ([#1197](https://github.com/TheOpenDictionary/odict/issues/1197)) ([026e2f2](https://github.com/TheOpenDictionary/odict/commit/026e2f292bf02c182684f8656b9eaa13533f1a72))
* Add `tags` fields to `sense` and `form`, as well as move `lemma` to `<sense>` ([#1192](https://github.com/TheOpenDictionary/odict/issues/1192)) ([f03d8c1](https://github.com/TheOpenDictionary/odict/commit/f03d8c122f96ec20f85ccff639962c9ffa44aee5))
* Add media URLs to `<entry>` nodes ([#1202](https://github.com/TheOpenDictionary/odict/issues/1202)) ([04c2307](https://github.com/TheOpenDictionary/odict/commit/04c2307b005ff57bbe460f7b1034f97c811b580f))
* Add translation node ([#1196](https://github.com/TheOpenDictionary/odict/issues/1196)) ([ef15aba](https://github.com/TheOpenDictionary/odict/commit/ef15abaf0749cf014315fb57ec63c50d2ae59e98))
* Move `translations` and `forms` to `sense` ([#1200](https://github.com/TheOpenDictionary/odict/issues/1200)) ([5c0e746](https://github.com/TheOpenDictionary/odict/commit/5c0e7466df84f5435cc53eba7fcc72813c11d28c))
* Remove wrapper components ([0908f01](https://github.com/TheOpenDictionary/odict/commit/0908f0128c1dd1b0749b756d757d8f3aa50e6c1c))
* Use `structural_convert` macro instead of custom From implementations ([#1199](https://github.com/TheOpenDictionary/odict/issues/1199)) ([392d624](https://github.com/TheOpenDictionary/odict/commit/392d624a4b956f0bc22d0529b4ccb0307807cdfd))

## [1.4.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.3.0...python/v1.4.0) (2025-04-20)


### Features

* Add support for word forms and lemmas ([#1186](https://github.com/TheOpenDictionary/odict/issues/1186)) ([9e37a28](https://github.com/TheOpenDictionary/odict/commit/9e37a2834fda82bfaf558aeab9cc74fbced5a1d4))

## [1.3.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.2.0...python/v1.3.0) (2025-04-13)


### Features

* Add flag to support case insensitivity for lookups and tokenization ([#1179](https://github.com/TheOpenDictionary/odict/issues/1179)) ([a7e7baa](https://github.com/TheOpenDictionary/odict/commit/a7e7baac0f8d02e565a2d01acdc59c9bd1bc3242))

## [1.2.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.1.1...python/v1.2.0) (2025-04-04)


### Features

* **tokenize:** Add additional metadata to Token model ([bd44701](https://github.com/TheOpenDictionary/odict/commit/bd44701bb3ef59fafac31a2b6582c729fd881f1e))

## [1.1.1](https://github.com/TheOpenDictionary/odict/compare/python/v1.1.0...python/v1.1.1) (2025-04-03)


### Bug Fixes

* Update Python version ([f9ec497](https://github.com/TheOpenDictionary/odict/commit/f9ec4972f3906185863dd9cdac5d02306292c483))

## [1.0.0](https://github.com/TheOpenDictionary/odict/compare/python-v1.0.0...python/v1.0.0) (2025-04-03)


### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))


### Bug Fixes

* **deps:** Update rust crate pyo3 to 0.24.0 ([#1141](https://github.com/TheOpenDictionary/odict/issues/1141)) ([3fb50bd](https://github.com/TheOpenDictionary/odict/commit/3fb50bd371fae1163e2f0acdb2c68e4692555d94))

## [1.0.0](https://github.com/TheOpenDictionary/odict/compare/python-v1.0.0...python-v1.0.0) (2024-12-25)


### Features

* Add indexing ([#656](https://github.com/TheOpenDictionary/odict/issues/656)) ([a94db99](https://github.com/TheOpenDictionary/odict/commit/a94db9953c34df96bedff5c3ebde989a64d27ace))
* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* Rename Python library ([928343a](https://github.com/TheOpenDictionary/odict/commit/928343a7df53d64aa25d7e262f21f4aa0f09cc5e))
* Swap asdf with mise ([1451356](https://github.com/TheOpenDictionary/odict/commit/145135680138d5438e98d1f1d61a9b82edba9c7c))
* Update Python package version ([1980388](https://github.com/TheOpenDictionary/odict/commit/19803884381c9f8e6483e35d73f93351529950e1))
