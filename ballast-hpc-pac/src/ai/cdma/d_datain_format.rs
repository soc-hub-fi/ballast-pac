#[doc = "Register `D_DATAIN_FORMAT` reader"]
pub struct R(crate::R<D_DATAIN_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAIN_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAIN_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAIN_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DATAIN_FORMAT` writer"]
pub struct W(crate::W<D_DATAIN_FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DATAIN_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D_DATAIN_FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DATAIN_FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAIN_FORMAT_A {
    #[doc = "1: `1`"]
    PIXEL = 1,
    #[doc = "0: `0`"]
    FEATURE = 0,
}
impl From<DATAIN_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DATAIN_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAIN_FORMAT` reader - "]
pub struct DATAIN_FORMAT_R(crate::FieldReader<bool>);
impl DATAIN_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAIN_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAIN_FORMAT_A {
        match self.bits {
            true => DATAIN_FORMAT_A::PIXEL,
            false => DATAIN_FORMAT_A::FEATURE,
        }
    }
    #[doc = "Checks if the value of the field is `PIXEL`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        **self == DATAIN_FORMAT_A::PIXEL
    }
    #[doc = "Checks if the value of the field is `FEATURE`"]
    #[inline(always)]
    pub fn is_feature(&self) -> bool {
        **self == DATAIN_FORMAT_A::FEATURE
    }
}
impl core::ops::Deref for DATAIN_FORMAT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIN_FORMAT` writer - "]
pub struct DATAIN_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIN_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAIN_FORMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut W {
        self.variant(DATAIN_FORMAT_A::PIXEL)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn feature(self) -> &'a mut W {
        self.variant(DATAIN_FORMAT_A::FEATURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIXEL_FORMAT_A {
    #[doc = "0: `0`"]
    T_R8 = 0,
    #[doc = "1: `1`"]
    T_R10 = 1,
    #[doc = "2: `10`"]
    T_R12 = 2,
    #[doc = "3: `11`"]
    T_R16 = 3,
    #[doc = "4: `100`"]
    T_R16_I = 4,
    #[doc = "5: `101`"]
    T_R16_F = 5,
    #[doc = "6: `110`"]
    T_A16B16G16R16 = 6,
    #[doc = "7: `111`"]
    T_X16B16G16R16 = 7,
    #[doc = "8: `1000`"]
    T_A16B16G16R16_F = 8,
    #[doc = "9: `1001`"]
    T_A16Y16U16V16 = 9,
    #[doc = "10: `1010`"]
    T_V16U16Y16A16 = 10,
    #[doc = "11: `1011`"]
    T_A16Y16U16V16_F = 11,
    #[doc = "12: `1100`"]
    T_A8B8G8R8 = 12,
    #[doc = "31: `11111`"]
    T_Y10___V10U10_N444 = 31,
    #[doc = "19: `10011`"]
    T_R8G8B8X8 = 19,
    #[doc = "13: `1101`"]
    T_A8R8G8B8 = 13,
    #[doc = "26: `11010`"]
    T_A8Y8U8V8 = 26,
    #[doc = "18: `10010`"]
    T_B8G8R8X8 = 18,
    #[doc = "32: `100000`"]
    T_Y12___U12V12_N444 = 32,
    #[doc = "20: `10100`"]
    T_A2B10G10R10 = 20,
    #[doc = "14: `1110`"]
    T_B8G8R8A8 = 14,
    #[doc = "33: `100001`"]
    T_Y12___V12U12_N444 = 33,
    #[doc = "21: `10101`"]
    T_A2R10G10B10 = 21,
    #[doc = "15: `1111`"]
    T_R8G8B8A8 = 15,
    #[doc = "34: `100010`"]
    T_Y16___U16V16_N444 = 34,
    #[doc = "22: `10110`"]
    T_B10G10R10A2 = 22,
    #[doc = "27: `11011`"]
    T_V8U8Y8A8 = 27,
    #[doc = "16: `10000`"]
    T_X8B8G8R8 = 16,
    #[doc = "35: `100011`"]
    T_Y16___V16U16_N444 = 35,
    #[doc = "23: `10111`"]
    T_R10G10B10A2 = 23,
    #[doc = "28: `11100`"]
    T_Y8___U8V8_N444 = 28,
    #[doc = "29: `11101`"]
    T_Y8___V8U8_N444 = 29,
    #[doc = "30: `11110`"]
    T_Y10___U10V10_N444 = 30,
    #[doc = "17: `10001`"]
    T_X8R8G8B8 = 17,
    #[doc = "24: `11000`"]
    T_A2Y10U10V10 = 24,
    #[doc = "25: `11001`"]
    T_V10U10Y10A2 = 25,
}
impl From<PIXEL_FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PIXEL_FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PIXEL_FORMAT` reader - "]
pub struct PIXEL_FORMAT_R(crate::FieldReader<u8>);
impl PIXEL_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIXEL_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIXEL_FORMAT_A> {
        match self.bits {
            0 => Some(PIXEL_FORMAT_A::T_R8),
            1 => Some(PIXEL_FORMAT_A::T_R10),
            2 => Some(PIXEL_FORMAT_A::T_R12),
            3 => Some(PIXEL_FORMAT_A::T_R16),
            4 => Some(PIXEL_FORMAT_A::T_R16_I),
            5 => Some(PIXEL_FORMAT_A::T_R16_F),
            6 => Some(PIXEL_FORMAT_A::T_A16B16G16R16),
            7 => Some(PIXEL_FORMAT_A::T_X16B16G16R16),
            8 => Some(PIXEL_FORMAT_A::T_A16B16G16R16_F),
            9 => Some(PIXEL_FORMAT_A::T_A16Y16U16V16),
            10 => Some(PIXEL_FORMAT_A::T_V16U16Y16A16),
            11 => Some(PIXEL_FORMAT_A::T_A16Y16U16V16_F),
            12 => Some(PIXEL_FORMAT_A::T_A8B8G8R8),
            31 => Some(PIXEL_FORMAT_A::T_Y10___V10U10_N444),
            19 => Some(PIXEL_FORMAT_A::T_R8G8B8X8),
            13 => Some(PIXEL_FORMAT_A::T_A8R8G8B8),
            26 => Some(PIXEL_FORMAT_A::T_A8Y8U8V8),
            18 => Some(PIXEL_FORMAT_A::T_B8G8R8X8),
            32 => Some(PIXEL_FORMAT_A::T_Y12___U12V12_N444),
            20 => Some(PIXEL_FORMAT_A::T_A2B10G10R10),
            14 => Some(PIXEL_FORMAT_A::T_B8G8R8A8),
            33 => Some(PIXEL_FORMAT_A::T_Y12___V12U12_N444),
            21 => Some(PIXEL_FORMAT_A::T_A2R10G10B10),
            15 => Some(PIXEL_FORMAT_A::T_R8G8B8A8),
            34 => Some(PIXEL_FORMAT_A::T_Y16___U16V16_N444),
            22 => Some(PIXEL_FORMAT_A::T_B10G10R10A2),
            27 => Some(PIXEL_FORMAT_A::T_V8U8Y8A8),
            16 => Some(PIXEL_FORMAT_A::T_X8B8G8R8),
            35 => Some(PIXEL_FORMAT_A::T_Y16___V16U16_N444),
            23 => Some(PIXEL_FORMAT_A::T_R10G10B10A2),
            28 => Some(PIXEL_FORMAT_A::T_Y8___U8V8_N444),
            29 => Some(PIXEL_FORMAT_A::T_Y8___V8U8_N444),
            30 => Some(PIXEL_FORMAT_A::T_Y10___U10V10_N444),
            17 => Some(PIXEL_FORMAT_A::T_X8R8G8B8),
            24 => Some(PIXEL_FORMAT_A::T_A2Y10U10V10),
            25 => Some(PIXEL_FORMAT_A::T_V10U10Y10A2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T_R8`"]
    #[inline(always)]
    pub fn is_t_r8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R8
    }
    #[doc = "Checks if the value of the field is `T_R10`"]
    #[inline(always)]
    pub fn is_t_r10(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R10
    }
    #[doc = "Checks if the value of the field is `T_R12`"]
    #[inline(always)]
    pub fn is_t_r12(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R12
    }
    #[doc = "Checks if the value of the field is `T_R16`"]
    #[inline(always)]
    pub fn is_t_r16(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R16
    }
    #[doc = "Checks if the value of the field is `T_R16_I`"]
    #[inline(always)]
    pub fn is_t_r16_i(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R16_I
    }
    #[doc = "Checks if the value of the field is `T_R16_F`"]
    #[inline(always)]
    pub fn is_t_r16_f(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R16_F
    }
    #[doc = "Checks if the value of the field is `T_A16B16G16R16`"]
    #[inline(always)]
    pub fn is_t_a16b16g16r16(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A16B16G16R16
    }
    #[doc = "Checks if the value of the field is `T_X16B16G16R16`"]
    #[inline(always)]
    pub fn is_t_x16b16g16r16(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_X16B16G16R16
    }
    #[doc = "Checks if the value of the field is `T_A16B16G16R16_F`"]
    #[inline(always)]
    pub fn is_t_a16b16g16r16_f(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A16B16G16R16_F
    }
    #[doc = "Checks if the value of the field is `T_A16Y16U16V16`"]
    #[inline(always)]
    pub fn is_t_a16y16u16v16(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A16Y16U16V16
    }
    #[doc = "Checks if the value of the field is `T_V16U16Y16A16`"]
    #[inline(always)]
    pub fn is_t_v16u16y16a16(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_V16U16Y16A16
    }
    #[doc = "Checks if the value of the field is `T_A16Y16U16V16_F`"]
    #[inline(always)]
    pub fn is_t_a16y16u16v16_f(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A16Y16U16V16_F
    }
    #[doc = "Checks if the value of the field is `T_A8B8G8R8`"]
    #[inline(always)]
    pub fn is_t_a8b8g8r8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A8B8G8R8
    }
    #[doc = "Checks if the value of the field is `T_Y10___V10U10_N444`"]
    #[inline(always)]
    pub fn is_t_y10___v10u10_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y10___V10U10_N444
    }
    #[doc = "Checks if the value of the field is `T_R8G8B8X8`"]
    #[inline(always)]
    pub fn is_t_r8g8b8x8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R8G8B8X8
    }
    #[doc = "Checks if the value of the field is `T_A8R8G8B8`"]
    #[inline(always)]
    pub fn is_t_a8r8g8b8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A8R8G8B8
    }
    #[doc = "Checks if the value of the field is `T_A8Y8U8V8`"]
    #[inline(always)]
    pub fn is_t_a8y8u8v8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A8Y8U8V8
    }
    #[doc = "Checks if the value of the field is `T_B8G8R8X8`"]
    #[inline(always)]
    pub fn is_t_b8g8r8x8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_B8G8R8X8
    }
    #[doc = "Checks if the value of the field is `T_Y12___U12V12_N444`"]
    #[inline(always)]
    pub fn is_t_y12___u12v12_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y12___U12V12_N444
    }
    #[doc = "Checks if the value of the field is `T_A2B10G10R10`"]
    #[inline(always)]
    pub fn is_t_a2b10g10r10(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A2B10G10R10
    }
    #[doc = "Checks if the value of the field is `T_B8G8R8A8`"]
    #[inline(always)]
    pub fn is_t_b8g8r8a8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_B8G8R8A8
    }
    #[doc = "Checks if the value of the field is `T_Y12___V12U12_N444`"]
    #[inline(always)]
    pub fn is_t_y12___v12u12_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y12___V12U12_N444
    }
    #[doc = "Checks if the value of the field is `T_A2R10G10B10`"]
    #[inline(always)]
    pub fn is_t_a2r10g10b10(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A2R10G10B10
    }
    #[doc = "Checks if the value of the field is `T_R8G8B8A8`"]
    #[inline(always)]
    pub fn is_t_r8g8b8a8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R8G8B8A8
    }
    #[doc = "Checks if the value of the field is `T_Y16___U16V16_N444`"]
    #[inline(always)]
    pub fn is_t_y16___u16v16_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y16___U16V16_N444
    }
    #[doc = "Checks if the value of the field is `T_B10G10R10A2`"]
    #[inline(always)]
    pub fn is_t_b10g10r10a2(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_B10G10R10A2
    }
    #[doc = "Checks if the value of the field is `T_V8U8Y8A8`"]
    #[inline(always)]
    pub fn is_t_v8u8y8a8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_V8U8Y8A8
    }
    #[doc = "Checks if the value of the field is `T_X8B8G8R8`"]
    #[inline(always)]
    pub fn is_t_x8b8g8r8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_X8B8G8R8
    }
    #[doc = "Checks if the value of the field is `T_Y16___V16U16_N444`"]
    #[inline(always)]
    pub fn is_t_y16___v16u16_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y16___V16U16_N444
    }
    #[doc = "Checks if the value of the field is `T_R10G10B10A2`"]
    #[inline(always)]
    pub fn is_t_r10g10b10a2(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_R10G10B10A2
    }
    #[doc = "Checks if the value of the field is `T_Y8___U8V8_N444`"]
    #[inline(always)]
    pub fn is_t_y8___u8v8_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y8___U8V8_N444
    }
    #[doc = "Checks if the value of the field is `T_Y8___V8U8_N444`"]
    #[inline(always)]
    pub fn is_t_y8___v8u8_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y8___V8U8_N444
    }
    #[doc = "Checks if the value of the field is `T_Y10___U10V10_N444`"]
    #[inline(always)]
    pub fn is_t_y10___u10v10_n444(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_Y10___U10V10_N444
    }
    #[doc = "Checks if the value of the field is `T_X8R8G8B8`"]
    #[inline(always)]
    pub fn is_t_x8r8g8b8(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_X8R8G8B8
    }
    #[doc = "Checks if the value of the field is `T_A2Y10U10V10`"]
    #[inline(always)]
    pub fn is_t_a2y10u10v10(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_A2Y10U10V10
    }
    #[doc = "Checks if the value of the field is `T_V10U10Y10A2`"]
    #[inline(always)]
    pub fn is_t_v10u10y10a2(&self) -> bool {
        **self == PIXEL_FORMAT_A::T_V10U10Y10A2
    }
}
impl core::ops::Deref for PIXEL_FORMAT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIXEL_FORMAT` writer - "]
pub struct PIXEL_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL_FORMAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t_r8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t_r10(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R10)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t_r12(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R12)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t_r16(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R16)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn t_r16_i(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R16_I)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn t_r16_f(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R16_F)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn t_a16b16g16r16(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A16B16G16R16)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn t_x16b16g16r16(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_X16B16G16R16)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn t_a16b16g16r16_f(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A16B16G16R16_F)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn t_a16y16u16v16(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A16Y16U16V16)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn t_v16u16y16a16(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_V16U16Y16A16)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn t_a16y16u16v16_f(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A16Y16U16V16_F)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn t_a8b8g8r8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A8B8G8R8)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn t_y10___v10u10_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y10___V10U10_N444)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn t_r8g8b8x8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R8G8B8X8)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn t_a8r8g8b8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A8R8G8B8)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn t_a8y8u8v8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A8Y8U8V8)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn t_b8g8r8x8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_B8G8R8X8)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn t_y12___u12v12_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y12___U12V12_N444)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn t_a2b10g10r10(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A2B10G10R10)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn t_b8g8r8a8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_B8G8R8A8)
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn t_y12___v12u12_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y12___V12U12_N444)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn t_a2r10g10b10(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A2R10G10B10)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn t_r8g8b8a8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R8G8B8A8)
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn t_y16___u16v16_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y16___U16V16_N444)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn t_b10g10r10a2(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_B10G10R10A2)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn t_v8u8y8a8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_V8U8Y8A8)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn t_x8b8g8r8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_X8B8G8R8)
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn t_y16___v16u16_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y16___V16U16_N444)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn t_r10g10b10a2(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_R10G10B10A2)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn t_y8___u8v8_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y8___U8V8_N444)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn t_y8___v8u8_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y8___V8U8_N444)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn t_y10___u10v10_n444(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_Y10___U10V10_N444)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn t_x8r8g8b8(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_X8R8G8B8)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn t_a2y10u10v10(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_A2Y10U10V10)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn t_v10u10y10a2(self) -> &'a mut W {
        self.variant(PIXEL_FORMAT_A::T_V10U10Y10A2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL_MAPPING_A {
    #[doc = "0: `0`"]
    PITCH_LINEAR = 0,
    #[doc = "1: `1`"]
    RESERVED_LINEAR = 1,
}
impl From<PIXEL_MAPPING_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_MAPPING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_MAPPING` reader - "]
pub struct PIXEL_MAPPING_R(crate::FieldReader<bool>);
impl PIXEL_MAPPING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIXEL_MAPPING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_MAPPING_A {
        match self.bits {
            false => PIXEL_MAPPING_A::PITCH_LINEAR,
            true => PIXEL_MAPPING_A::RESERVED_LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `PITCH_LINEAR`"]
    #[inline(always)]
    pub fn is_pitch_linear(&self) -> bool {
        **self == PIXEL_MAPPING_A::PITCH_LINEAR
    }
    #[doc = "Checks if the value of the field is `RESERVED_LINEAR`"]
    #[inline(always)]
    pub fn is_reserved_linear(&self) -> bool {
        **self == PIXEL_MAPPING_A::RESERVED_LINEAR
    }
}
impl core::ops::Deref for PIXEL_MAPPING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIXEL_MAPPING` writer - "]
pub struct PIXEL_MAPPING_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL_MAPPING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL_MAPPING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pitch_linear(self) -> &'a mut W {
        self.variant(PIXEL_MAPPING_A::PITCH_LINEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reserved_linear(self) -> &'a mut W {
        self.variant(PIXEL_MAPPING_A::RESERVED_LINEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL_SIGN_OVERRIDE_A {
    #[doc = "0: `0`"]
    UNSIGNED_INT = 0,
    #[doc = "1: `1`"]
    SIGNED_INT = 1,
}
impl From<PIXEL_SIGN_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_SIGN_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_SIGN_OVERRIDE` reader - "]
pub struct PIXEL_SIGN_OVERRIDE_R(crate::FieldReader<bool>);
impl PIXEL_SIGN_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIXEL_SIGN_OVERRIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_SIGN_OVERRIDE_A {
        match self.bits {
            false => PIXEL_SIGN_OVERRIDE_A::UNSIGNED_INT,
            true => PIXEL_SIGN_OVERRIDE_A::SIGNED_INT,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED_INT`"]
    #[inline(always)]
    pub fn is_unsigned_int(&self) -> bool {
        **self == PIXEL_SIGN_OVERRIDE_A::UNSIGNED_INT
    }
    #[doc = "Checks if the value of the field is `SIGNED_INT`"]
    #[inline(always)]
    pub fn is_signed_int(&self) -> bool {
        **self == PIXEL_SIGN_OVERRIDE_A::SIGNED_INT
    }
}
impl core::ops::Deref for PIXEL_SIGN_OVERRIDE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIXEL_SIGN_OVERRIDE` writer - "]
pub struct PIXEL_SIGN_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL_SIGN_OVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL_SIGN_OVERRIDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unsigned_int(self) -> &'a mut W {
        self.variant(PIXEL_SIGN_OVERRIDE_A::UNSIGNED_INT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn signed_int(self) -> &'a mut W {
        self.variant(PIXEL_SIGN_OVERRIDE_A::SIGNED_INT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_format(&self) -> DATAIN_FORMAT_R {
        DATAIN_FORMAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pixel_format(&self) -> PIXEL_FORMAT_R {
        PIXEL_FORMAT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pixel_mapping(&self) -> PIXEL_MAPPING_R {
        PIXEL_MAPPING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pixel_sign_override(&self) -> PIXEL_SIGN_OVERRIDE_R {
        PIXEL_SIGN_OVERRIDE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_format(&mut self) -> DATAIN_FORMAT_W {
        DATAIN_FORMAT_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pixel_format(&mut self) -> PIXEL_FORMAT_W {
        PIXEL_FORMAT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pixel_mapping(&mut self) -> PIXEL_MAPPING_W {
        PIXEL_MAPPING_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pixel_sign_override(&mut self) -> PIXEL_SIGN_OVERRIDE_W {
        PIXEL_SIGN_OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input data format and pixel format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_datain_format](index.html) module"]
pub struct D_DATAIN_FORMAT_SPEC;
impl crate::RegisterSpec for D_DATAIN_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_datain_format::R](R) reader structure"]
impl crate::Readable for D_DATAIN_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_datain_format::W](W) writer structure"]
impl crate::Writable for D_DATAIN_FORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DATAIN_FORMAT to value 0x6000"]
impl crate::Resettable for D_DATAIN_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000
    }
}
