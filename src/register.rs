/// A read-write register of type T.
///
/// Contains one value of type T and provides volatile read/write functions to it.
///
/// At present this presents a significant unsafety issue by essentially providing a safe
/// wrapper to `read_volatile` and `write_volatile` without doing any other checking.
/// To be considered further.
pub struct RWRegister<T> {
    register: T,
}

impl<T> RWRegister<T> {
    /// Reads the value of the register.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.register as *const T) }
    }

    /// Writes a new value to the register.
    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe { ::core::ptr::write_volatile(&self.register as *const T as *mut T, val) }
    }
}

/// A read-only register of type T.
///
/// Contains one value of type T and provides a volatile read function to it.
///
/// At present this presents a significant unsafety issue by essentially providing a safe
/// wrapper to `read_volatile` without doing any other checking.  To be considered further.
pub struct RORegister<T> {
    register: T,
}

impl<T> RORegister<T> {
    /// Reads the value of the register.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.register as *const T) }
    }
}

/// Write to a RWRegister.
///
/// Usage example:
/// ```
/// // Write some value to the register.
/// write_reg!(stm32f405::gpio, GPIOA.odr, 1<<3);
/// // Write values to specific fields. Unspecified fields are written to 0.
/// write_reg!(stm32f405::gpio, GPIOA.moder, MODER3: Output, MODER4: Analog);
/// ```
///
/// Like `modify_reg!`, this macro can be used in two ways, either with a single value to write to
/// the whole register, or with multiple fields each with their own value.
///
/// In both cases, the first argument is the path to the peripheral module, e.g.,
/// `stm32f405::gpio`, and the second argument is the instance of that peripheral
/// and register you wish you access, e.g., `GPIOA.moder`.
///
/// In the single-value usage, the final argument is just the value to write:
/// ```
/// // Turn on PA3 (and turn everything else off).
/// write_reg!(stm32f405::gpio, GPIOA.odr, 1<<3);
/// ```
///
/// Otherwise, the remaining arguments are each `Field: Value` pairs:
/// ```
/// // Set PA3 to Output, PA4 to Analog, and everything else to 0 (which is Input).
/// write_reg!(stm32f405::gpio, GPIOA.moder, MODER3: 0b01, MODER4: 0b11);
/// ```
/// For fields with annotated values, you can also specify a named value:
/// ```
/// // As above, but with named values.
/// write_reg!(stm32f405::gpio, GPIOA.moder, MODER3: Output, MODER4: Analog);
/// ```
///
/// This macro expands to calling `peripheral::instance.register.write(value)`,
/// where in the second usage, the value is computed as the bitwise OR of
/// each field value, which are masked and shifted appropriately for the given field.
/// The named values are brought into scope by `use peripheral::register::field::*` for
/// each field. The same constants could just be specified manually:
/// ```
/// // As above, but being explicit about named values.
/// write_reg!(stm32f405::gpio, GPIOA.moder, MODER3: stm32f405::gpio::moder::MODER3::Output,
///                                          MODER4: stm32f405::gpio::moder::MODER4::Analog);
/// ```
///
/// The fully expanded form is equivalent to:
/// ```
/// // As above, but expanded.
/// stm32f405::gpio::GPIOA.moder.write(
///     ((stm32f405::gpio::moder::MODER3::Output & stm32f405::gpio::moder::MODER3::_mask)
///      << stm32f405::gpio::moder::MODER3::_offset)
///     |
///     ((stm32f405::gpio::moder::MODER4::Analog & stm32f405::gpio::moder::MODER4::_mask)
///      << stm32f405::gpio::moder::MODER4::_offset)
/// );
/// ```
#[macro_export]
macro_rules! write_reg {
    ( $periph:path, $instance:ident . $reg:ident, $( $field:ident : $value:expr ),+ ) => {{
        use $periph as periph;
        periph::$instance.$reg.write(
            $({ use $periph::{$reg::$field::*}; ($value & _mask) << _offset }) | *
        );
    }};
    ( $periph:path, $instance:ident . $reg:ident, $value:expr ) => {{
        use $periph as periph;
        $periph::$instance.$reg.write($value);
    }};
}

/// Modify a RWRegister.
///
/// Usage example:
/// ```
/// // Update the register to ensure bit 3 is set.
/// modify_reg!(stm32f405::gpio, GPIOA.odr, |reg| reg | (1<<3));
/// // Write values to specific fields. Unspecified fields are left unchanged.
/// modify_reg!(stm32f405::gpio, GPIOA.moder, MODER3: Output, MODER4: Analog);
/// ```
///
/// Like `write_reg!`, this macro can be used in two ways, either with a modification of the entire
/// register, or by specifying which fields to change and what value to change them to.
///
/// In both cases, the first argument is the path to the peripheral module, e.g.,
/// `stm32f405::gpio`, and the second argument is the instance of that peripheral
/// and register you wish you access, e.g., `GPIOA.moder`.
///
/// In the whole-register usage, the final argument is a closure that accepts the current value
/// of the register and returns the new value to write:
/// ```
/// // Turn on PA3 without affecting anything else.
/// modify_reg!(stm32f405::gpio, GPIOA.odr, |reg| reg | (1<<3));
/// ```
///
/// Otherwise, the remaining arguments are `Field: Value` pairs:
/// ```
/// // Set PA3 to Output, PA4 to Analog, and leave everything else unchanged.
/// modify_reg!(stm32f405::gpio, GPIOA.moder, MODER3: 0b01, MODER4: 0b11);
/// ```
///
/// For fields with annotated values, you can also specify a named value:
/// ```
/// // As above, but with named values.
/// modify_reg!(stm32f405::gpio, GPIOA.moder, MODER3: Output, MODER4: Analog);
/// ```
///
/// This macro expands to calling `peripheral::instance.register.write(value)`.
/// When called with a closure, `peripheral::instance.register.read()` is called, the result
/// passed in to the closure, and the return value of the closure is used for `value`.
/// When called with `Field: Value` arguments, the current value is read and then masked
/// according to the specified fields, and then ORd with the OR of each field value,
/// each masked and shifted appropriately for the field. The named values are brought into scope
/// by `use peripheral::register::field::*` for each field. The same constants could just be
/// specified manually:
/// ```
/// // As above, but being explicit about named values.
/// modify_reg!(stm32f405::gpio, GPIOA.moder, MODER3: stm32f405::gpio::moder::MODER3::Output,
///                                           MODER4: stm32f405::gpio::moder::MODER4::Analog);
/// ```
///
/// The fully expanded form is equivalent to:
/// ```
/// // As above, but expanded.
/// stm32f405::gpio::GPIOA.moder.write(
///     (
///         // First read the current value...
///         stm32f405::gpio::GPIOA.moder.read()
///         // Then AND it with an appropriate mask...
///         &
///         !(
///             (stm32f405::gpio::moder::MODER3::_mask << stm32f405::gpio::moder::MODER3::_offset)
///             |
///             (stm32f405::gpio::moder::MODER4::_mask << stm32f405::gpio::moder::MODER4::_offset)
///         )
///     )
///     // Then OR with each field value.
///     |
///         ((stm32f405::gpio::moder::MODER3::Output & stm32f405::gpio::moder::MODER3::_mask)
///          << stm32f405::gpio::moder::MODER3::_offset)
///     |
///         ((stm32f405::gpio::moder::MODER4::Analog & stm32f405::gpio::moder::MODER3::_mask)
///          << stm32f405::gpio::moder::MODER3::_offset)
/// );
/// ```
#[macro_export]
macro_rules! modify_reg {
    ( $periph:path, $instance:ident . $reg:ident, $( $field:ident : $value:expr ),+ ) => {{
        use $periph as periph;
        periph::$instance.$reg.write(
            (periph::$instance.$reg.read() & !( $({ use $periph::{$reg::$field::*}; _mask << _offset }) | * ))
            | $({ use $periph::{$reg::$field::*}; ($value & _mask) << _offset }) | *);
    }};
    ( $periph:path, $instance:ident . $reg:ident, $fn:expr ) => {{
        use $periph as periph;
        periph::$instance.$reg.write($fn($periph::$instance.$reg.read()));
    }};
}

/// Read the value from a register.
///
/// Usage example:
/// ```
/// // Read the whole register.
/// let val = read_reg!(stm32f405::gpio, GPIOA.idr);
///
/// // Read one field from the register.
/// let val = read_reg!(stm32f405::gpio, GPIOA.idr, IDR2);
///
/// // Check if one field is equal to a specific value, with the field's named values in scope.
/// while read_reg!(stm32f405::gpio, GPIOA.idr, IDR2 == High) {}
/// ```
///
/// Like `write_reg!`, this macro can be used multiple ways, either reading the entire register or
/// reading a single field from it and potentially performing a comparison with that field.
///
/// In all cases, the first argument is the path to the peripheral module, e.g.,
/// `stm32f405::gpio`, and the second argument is the instance of that peripheral and the register
/// you wish to access, e.g., `GPIOA.idr`.
///
/// In the whole-register usage, the macro simply returns the register's value:
/// ```
/// // Read the entire value of GPIOA.IDR into `val`.
/// let val = read_reg!(stm32f405::gpio, GPIOA.idr);
/// ```
///
/// For reading individual fields, the macro masks and shifts appropriately:
/// ```
/// // Read just the value of the field GPIOA.IDR2 into `val`.
/// let val = read_reg!(stm32f405::gpio, GPIOA.idr, IDR2);
///
/// // As above, but expanded for exposition:
/// let val = (stm32f405::gpio::GPIOA.idr.read() >> stm32f405::gpio::idr::IDR2::_offset)
///           & stm32f405::gpio::idr::IDR2::_mask;
/// ```
///
/// For comparing individual fields, the macro masks and shifts and then performs the comparison:
/// ```
/// // Loop while PA2 is High.
/// while read_reg!(stm32f405::gpio, GPIOA.idr, IDR2 == High) {}
///
/// // Only proceed if the clock is not the HSI.
/// if read_reg!(stm32f405::rcc, RCC.cfgr, SWS != HSI) { }
///
/// // Equivalent expansion:
/// if ((stm32f405::rcc::RCC.cfgr.read() >> stm32f405::rcc::cfgr::SWS::_offset)
///     & stm32f405::rcc::cfgr::SYS::_mask) != stm32f405::rcc:cfgr::SYS::HSI { }
/// ```
#[macro_export]
macro_rules! read_reg {
    ( $periph:path, $instance:ident . $reg:ident, $field:ident ) => {{
        use $periph as periph;
        use $periph::{$reg::$field::*};
        (periph::$instance.$reg.read() >> _offset) & _mask
    }};
    ( $periph:path, $instance:ident . $reg:ident, $field:ident $($cmp:tt)* ) => {{
        use $periph as periph;
        use $periph::{$reg::$field::*};
        ((periph::$instance.$reg.read() >> _offset) & _mask) $($cmp)*
    }};
    ( $periph:path, $instance:ident . $reg:ident ) => {{
        use $periph as periph;
        periph::$instance.$reg.read()
    }};
}
