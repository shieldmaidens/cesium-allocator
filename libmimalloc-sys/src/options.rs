use std::ffi::c_long;

use crate::mi_option_t;

extern "C" {
    // Note: mi_option_{enable,disable} aren't exposed because they're redundant
    // and because of https://github.com/microsoft/mimalloc/issues/266.

    /// Returns true if the provided option is enabled.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;

    /// Enable or disable the given option.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], enables or disables the option. If it has,
    /// this function does nothing.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);

    /// Returns the value of the provided option.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed, see the mimalloc documentation for
    /// details: `<https://microsoft.github.io/mimalloc/group__options.html>`
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_get(option: mi_option_t) -> c_long;

    /// Set the option to the given value.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed,
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set(option: mi_option_t, value: c_long);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], sets the option to the given value. If it
    /// has, this function does nothing.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
}