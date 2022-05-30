#[doc = "Register `D_DATA_FORMAT` reader"]
pub struct R(crate::R<D_DATA_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATA_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATA_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATA_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DATA_FORMAT` writer"]
pub struct W(crate::W<D_DATA_FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DATA_FORMAT_SPEC>;
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
impl From<crate::W<D_DATA_FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DATA_FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT_DATA_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<INPUT_DATA_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_DATA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT_DATA` reader - "]
pub struct INPUT_DATA_R(crate::FieldReader<u8>);
impl INPUT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INPUT_DATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT_DATA_A> {
        match self.bits {
            2 => Some(INPUT_DATA_A::FP16),
            1 => Some(INPUT_DATA_A::INT16),
            0 => Some(INPUT_DATA_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        **self == INPUT_DATA_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        **self == INPUT_DATA_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        **self == INPUT_DATA_A::INT8
    }
}
impl core::ops::Deref for INPUT_DATA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_DATA` writer - "]
pub struct INPUT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT_DATA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(INPUT_DATA_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(INPUT_DATA_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(INPUT_DATA_A::INT8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn input_data(&self) -> INPUT_DATA_R {
        INPUT_DATA_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn input_data(&mut self) -> INPUT_DATA_W {
        INPUT_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Precision of input data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_data_format](index.html) module"]
pub struct D_DATA_FORMAT_SPEC;
impl crate::RegisterSpec for D_DATA_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_data_format::R](R) reader structure"]
impl crate::Readable for D_DATA_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_data_format::W](W) writer structure"]
impl crate::Writable for D_DATA_FORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DATA_FORMAT to value 0"]
impl crate::Resettable for D_DATA_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
