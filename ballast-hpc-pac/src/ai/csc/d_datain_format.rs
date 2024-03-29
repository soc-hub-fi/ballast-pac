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
#[doc = "Field `DATAIN_FORMAT` reader - "]
pub type DATAIN_FORMAT_R = crate::BitReader<DATAIN_FORMAT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DATAIN_FORMAT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DATAIN_FORMAT_A::PIXEL
    }
    #[doc = "Checks if the value of the field is `FEATURE`"]
    #[inline(always)]
    pub fn is_feature(&self) -> bool {
        *self == DATAIN_FORMAT_A::FEATURE
    }
}
#[doc = "Field `DATAIN_FORMAT` writer - "]
pub type DATAIN_FORMAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DATAIN_FORMAT_SPEC, DATAIN_FORMAT_A, O>;
impl<'a, const O: u8> DATAIN_FORMAT_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_format(&self) -> DATAIN_FORMAT_R {
        DATAIN_FORMAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn datain_format(&mut self) -> DATAIN_FORMAT_W<0> {
        DATAIN_FORMAT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DATAIN_FORMAT to value 0"]
impl crate::Resettable for D_DATAIN_FORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
