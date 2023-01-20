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
#[doc = "Field `PROC_PRECISION` reader - "]
pub type PROC_PRECISION_R = crate::FieldReader<u8, PROC_PRECISION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROC_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<PROC_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PROC_PRECISION_A) -> Self {
        variant as _
    }
}
impl PROC_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROC_PRECISION_A> {
        match self.bits {
            2 => Some(PROC_PRECISION_A::FP16),
            1 => Some(PROC_PRECISION_A::INT16),
            0 => Some(PROC_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == PROC_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == PROC_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == PROC_PRECISION_A::INT8
    }
}
#[doc = "Field `PROC_PRECISION` writer - "]
pub type PROC_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DATA_FORMAT_SPEC, u8, PROC_PRECISION_A, 2, O>;
impl<'a, const O: u8> PROC_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT8)
    }
}
#[doc = "Field `OUT_PRECISION` reader - "]
pub type OUT_PRECISION_R = crate::FieldReader<u8, OUT_PRECISION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<OUT_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_PRECISION_A) -> Self {
        variant as _
    }
}
impl OUT_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_PRECISION_A> {
        match self.bits {
            2 => Some(OUT_PRECISION_A::FP16),
            1 => Some(OUT_PRECISION_A::INT16),
            0 => Some(OUT_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == OUT_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == OUT_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == OUT_PRECISION_A::INT8
    }
}
#[doc = "Field `OUT_PRECISION` writer - "]
pub type OUT_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DATA_FORMAT_SPEC, u8, OUT_PRECISION_A, 2, O>;
impl<'a, const O: u8> OUT_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(OUT_PRECISION_A::INT8)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn out_precision(&self) -> OUT_PRECISION_R {
        OUT_PRECISION_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn proc_precision(&mut self) -> PROC_PRECISION_W<0> {
        PROC_PRECISION_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn out_precision(&mut self) -> OUT_PRECISION_W<2> {
        OUT_PRECISION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data precision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_data_format](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DATA_FORMAT to value 0"]
impl crate::Resettable for D_DATA_FORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
