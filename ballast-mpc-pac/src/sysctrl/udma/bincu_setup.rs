#[doc = "Register `BINCU_SETUP` reader"]
pub struct R(crate::R<BINCU_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINCU_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINCU_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINCU_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINCU_SETUP` writer"]
pub struct W(crate::W<BINCU_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINCU_SETUP_SPEC>;
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
impl From<crate::W<BINCU_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINCU_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINCU_SETUP` reader - "]
pub type BINCU_SETUP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BINCU_SETUP` writer - "]
pub type BINCU_SETUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BINCU_SETUP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bincu_setup(&self) -> BINCU_SETUP_R {
        BINCU_SETUP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bincu_setup(&mut self) -> BINCU_SETUP_W<0> {
        BINCU_SETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER binarization result count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bincu_setup](index.html) module"]
pub struct BINCU_SETUP_SPEC;
impl crate::RegisterSpec for BINCU_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bincu_setup::R](R) reader structure"]
impl crate::Readable for BINCU_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bincu_setup::W](W) writer structure"]
impl crate::Writable for BINCU_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BINCU_SETUP to value 0"]
impl crate::Resettable for BINCU_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
