#[doc = "Register `D_FLYING_MODE` reader"]
pub struct R(crate::R<D_FLYING_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FLYING_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FLYING_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FLYING_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_FLYING_MODE` writer"]
pub struct W(crate::W<D_FLYING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_FLYING_MODE_SPEC>;
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
impl From<crate::W<D_FLYING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_FLYING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLYING_MODE` reader - "]
pub type FLYING_MODE_R = crate::BitReader<FLYING_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLYING_MODE_A {
    #[doc = "1: `1`"]
    ON = 1,
    #[doc = "0: `0`"]
    OFF = 0,
}
impl From<FLYING_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLYING_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLYING_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLYING_MODE_A {
        match self.bits {
            true => FLYING_MODE_A::ON,
            false => FLYING_MODE_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FLYING_MODE_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FLYING_MODE_A::OFF
    }
}
#[doc = "Field `FLYING_MODE` writer - "]
pub type FLYING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_FLYING_MODE_SPEC, FLYING_MODE_A, O>;
impl<'a, const O: u8> FLYING_MODE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::ON)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FLYING_MODE_A::OFF)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FLYING_MODE_R {
        FLYING_MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn flying_mode(&mut self) -> FLYING_MODE_W<0> {
        FLYING_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_flying_mode](index.html) module"]
pub struct D_FLYING_MODE_SPEC;
impl crate::RegisterSpec for D_FLYING_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_flying_mode::R](R) reader structure"]
impl crate::Readable for D_FLYING_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_flying_mode::W](W) writer structure"]
impl crate::Writable for D_FLYING_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_FLYING_MODE to value 0"]
impl crate::Resettable for D_FLYING_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
