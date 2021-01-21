// {
// // The name of the package.
// // This must only contain alphanumeric, `-`, or `_` characters.
// "name": "foo",
// // The version of the package this row is describing.
// // This must be a valid version number according to the Semantic
// // Versioning 2.0.0 spec at https://semver.org/.
// "vers": "0.1.0",
// // Array of direct dependencies of the package.
// "deps": [
// {
// // Name of the dependency.
// // If the dependency is renamed from the original package name,
// // this is the new name. The original package name is stored in
// // the `package` field.
// "name": "rand",
// // The semver requirement for this dependency.
// // This must be a valid version requirement defined at
// // https://github.com/steveklabnik/semver#requirements.
// "req": "^0.6",
// // Array of features (as strings) enabled for this dependency.
// "features": ["i128_support"],
// // Boolean of whether or not this is an optional dependency.
// "optional": false,
// // Boolean of whether or not default features are enabled.
// "default_features": true,
// // The target platform for the dependency.
// // null if not a target dependency.
// // Otherwise, a string such as "cfg(windows)".
// "target": null,
// // The dependency kind.
// // "dev", "build", or "normal".
// // Note: this is a required field, but a small number of entries
// // exist in the crates.io index with either a missing or null
// // `kind` field due to implementation bugs.
// "kind": "normal",
// // The URL of the index of the registry where this dependency is
// // from as a string. If not specified or null, it is assumed the
// // dependency is in the current registry.
// "registry": null,
// // If the dependency is renamed, this is a string of the actual
// // package name. If not specified or null, this dependency is not
// // renamed.
// "package": null,
// }
// ],
// // A SHA256 checksum of the `.crate` file.
// "cksum": "d867001db0e2b6e0496f9fac96930e2d42233ecd3ca0413e0753d4c7695d289c",
// // Set of features defined for the package.
// // Each feature maps to an array of features or dependencies it enables.
// "features": {
// "extras": ["rand/simd_support"]
// },
// // Boolean of whether or not this version has been yanked.
// "yanked": false,
// // The `links` string value from the package's manifest, or null if not
// // specified. This field is optional and defaults to null.
// "links": null
// }
