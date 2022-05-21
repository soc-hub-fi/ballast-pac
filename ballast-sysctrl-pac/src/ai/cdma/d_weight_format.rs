#[doc = "Register `D_WEIGHT_FORMAT` reader"]
pub struct R(crate::R<D_WEIGHT_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_FORMAT` writer"]
pub struct W(crate::W<D_WEIGHT_FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_FORMAT_SPEC>;
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
impl From<crate::W<D_WEIGHT_FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEIGHT_FORMAT_A {
    #[doc = "1: `1`"]
    COMPRESSED = 1,
    #[doc = "0: `0`"]
    UNCOMPRESSED = 0,
}
impl From<WEIGHT_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: WEIGHT_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEIGHT_FORMAT` reader - "]
pub struct WEIGHT_FORMAT_R(crate::FieldReader<bool, WEIGHT_FORMAT_A>);
impl WEIGHT_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEIGHT_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEIGHT_FORMAT_A {
        match self.bits {
            true => WEIGHT_FORMAT_A::COMPRESSED,
            false => WEIGHT_FORMAT_A::UNCOMPRESSED,
        }
    }
    #[doc = "Checks if the value of the field is `COMPRESSED`"]
    #[inline(always)]
    pub fn is_compressed(&self) -> bool {
        **self == WEIGHT_FORMAT_A::COMPRESSED
    }
    #[doc = "Checks if the value of the field is `UNCOMPRESSED`"]
    #[inline(always)]
    pub fn is_uncompressed(&self) -> bool {
        **self == WEIGHT_FORMAT_A::UNCOMPRESSED
    }
}
impl core::ops::Deref for WEIGHT_FORMAT_R {
    type Target = crate::FieldReader<bool, WEIGHT_FORMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_FORMAT` writer - "]
pub struct WEIGHT_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WEIGHT_FORMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn compressed(self) -> &'a mut W {
        self.variant(WEIGHT_FORMAT_A::COMPRESSED)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uncompressed(self) -> &'a mut W {
        self.variant(WEIGHT_FORMAT_A::UNCOMPRESSED)
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn weight_format(&self) -> WEIGHT_FORMAT_R {
        WEIGHT_FORMAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn weight_format(&mut self) -> WEIGHT_FORMAT_W {
        WEIGHT_FORMAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether weight is compressed or not\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_format](index.html) module"]
pub struct D_WEIGHT_FORMAT_SPEC;
impl crate::RegisterSpec for D_WEIGHT_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_format::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_format::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_FORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_WEIGHT_FORMAT to value 0"]
impl crate::Resettable for D_WEIGHT_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
