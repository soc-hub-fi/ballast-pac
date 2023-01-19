#[doc = "Register `refclk` reader"]
pub struct R(crate::R<REFCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `refclk` writer"]
pub struct W(crate::W<REFCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLK_SPEC>;
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
impl From<crate::W<REFCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `drive_strength` reader - "]
pub type DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drive_strength` writer - "]
pub type DRIVE_STRENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, REFCLK_SPEC, u8, u8, 3, O>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, REFCLK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new(self.bits & 7)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W<0> {
        DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<3> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclk](index.html) module"]
pub struct REFCLK_SPEC;
impl crate::RegisterSpec for REFCLK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [refclk::R](R) reader structure"]
impl crate::Readable for REFCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refclk::W](W) writer structure"]
impl crate::Writable for REFCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets refclk to value 0x0f"]
impl crate::Resettable for REFCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
