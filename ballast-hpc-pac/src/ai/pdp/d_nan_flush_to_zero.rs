#[doc = "Register `D_NAN_FLUSH_TO_ZERO` reader"]
pub struct R(crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_NAN_FLUSH_TO_ZERO` writer"]
pub struct W(crate::W<D_NAN_FLUSH_TO_ZERO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_NAN_FLUSH_TO_ZERO_SPEC>;
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
impl From<crate::W<D_NAN_FLUSH_TO_ZERO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_NAN_FLUSH_TO_ZERO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NAN_TO_ZERO` reader - "]
pub type NAN_TO_ZERO_R = crate::BitReader<NAN_TO_ZERO_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAN_TO_ZERO_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<NAN_TO_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: NAN_TO_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl NAN_TO_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAN_TO_ZERO_A {
        match self.bits {
            true => NAN_TO_ZERO_A::ENABLE,
            false => NAN_TO_ZERO_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NAN_TO_ZERO_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NAN_TO_ZERO_A::DISABLE
    }
}
#[doc = "Field `NAN_TO_ZERO` writer - "]
pub type NAN_TO_ZERO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_NAN_FLUSH_TO_ZERO_SPEC, NAN_TO_ZERO_A, O>;
impl<'a, const O: u8> NAN_TO_ZERO_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NAN_TO_ZERO_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NAN_TO_ZERO_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nan_to_zero(&self) -> NAN_TO_ZERO_R {
        NAN_TO_ZERO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nan_to_zero(&mut self) -> NAN_TO_ZERO_W<0> {
        NAN_TO_ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Option to flush input NaN to zero\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_nan_flush_to_zero](index.html) module"]
pub struct D_NAN_FLUSH_TO_ZERO_SPEC;
impl crate::RegisterSpec for D_NAN_FLUSH_TO_ZERO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_nan_flush_to_zero::R](R) reader structure"]
impl crate::Readable for D_NAN_FLUSH_TO_ZERO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_nan_flush_to_zero::W](W) writer structure"]
impl crate::Writable for D_NAN_FLUSH_TO_ZERO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_NAN_FLUSH_TO_ZERO to value 0"]
impl crate::Resettable for D_NAN_FLUSH_TO_ZERO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
