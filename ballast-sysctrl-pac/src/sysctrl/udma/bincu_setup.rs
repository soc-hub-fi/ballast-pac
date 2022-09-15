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
pub struct BINCU_SETUP_R(crate::FieldReader<u32>);
impl BINCU_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BINCU_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINCU_SETUP_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BINCU_SETUP` writer - "]
pub struct BINCU_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BINCU_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
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
    pub fn bincu_setup(&mut self) -> BINCU_SETUP_W {
        BINCU_SETUP_W { w: self }
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
}
#[doc = "`reset()` method sets BINCU_SETUP to value 0"]
impl crate::Resettable for BINCU_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
