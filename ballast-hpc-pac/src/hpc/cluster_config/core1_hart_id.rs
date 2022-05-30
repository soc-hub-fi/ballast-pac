#[doc = "Register `core1_hart_id` reader"]
pub struct R(crate::R<CORE1_HART_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE1_HART_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE1_HART_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE1_HART_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core1_hart_id` writer"]
pub struct W(crate::W<CORE1_HART_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE1_HART_ID_SPEC>;
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
impl From<crate::W<CORE1_HART_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE1_HART_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core1_hart_id` reader - "]
pub struct CORE1_HART_ID_R(crate::FieldReader<u64>);
impl CORE1_HART_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        CORE1_HART_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE1_HART_ID_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `core1_hart_id` writer - "]
pub struct CORE1_HART_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE1_HART_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core1_hart_id(&self) -> CORE1_HART_ID_R {
        CORE1_HART_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core1_hart_id(&mut self) -> CORE1_HART_ID_W {
        CORE1_HART_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core#1 hart ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core1_hart_id](index.html) module"]
pub struct CORE1_HART_ID_SPEC;
impl crate::RegisterSpec for CORE1_HART_ID_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core1_hart_id::R](R) reader structure"]
impl crate::Readable for CORE1_HART_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core1_hart_id::W](W) writer structure"]
impl crate::Writable for CORE1_HART_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets core1_hart_id to value 0"]
impl crate::Resettable for CORE1_HART_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
