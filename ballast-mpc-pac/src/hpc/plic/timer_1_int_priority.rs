#[doc = "Register `timer_1_int_priority[%s]` reader"]
pub struct R(crate::R<TIMER_1_INT_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_1_INT_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_1_INT_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_1_INT_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer_1_int_priority[%s]` writer"]
pub struct W(crate::W<TIMER_1_INT_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_1_INT_PRIORITY_SPEC>;
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
impl From<crate::W<TIMER_1_INT_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_1_INT_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer_1_int_priority` reader - "]
pub struct TIMER_1_INT_PRIORITY_R(crate::FieldReader<u32>);
impl TIMER_1_INT_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_1_INT_PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_1_INT_PRIORITY_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer_1_int_priority` writer - "]
pub struct TIMER_1_INT_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_1_INT_PRIORITY_W<'a> {
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
    pub fn timer_1_int_priority(&self) -> TIMER_1_INT_PRIORITY_R {
        TIMER_1_INT_PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer_1_int_priority(&mut self) -> TIMER_1_INT_PRIORITY_W {
        TIMER_1_INT_PRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_1_int_priority](index.html) module"]
pub struct TIMER_1_INT_PRIORITY_SPEC;
impl crate::RegisterSpec for TIMER_1_INT_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_1_int_priority::R](R) reader structure"]
impl crate::Readable for TIMER_1_INT_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_1_int_priority::W](W) writer structure"]
impl crate::Writable for TIMER_1_INT_PRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets timer_1_int_priority[%s]
to value 0"]
impl crate::Resettable for TIMER_1_INT_PRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
