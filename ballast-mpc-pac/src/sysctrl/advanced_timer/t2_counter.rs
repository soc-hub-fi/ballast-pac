#[doc = "Register `T2_COUNTER` reader"]
pub struct R(crate::R<T2_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T2_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T2_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T2_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T2_COUNTER` writer"]
pub struct W(crate::W<T2_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T2_COUNTER_SPEC>;
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
impl From<crate::W<T2_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T2_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T2_COUNTER` reader - "]
pub struct T2_COUNTER_R(crate::FieldReader<u32, u32>);
impl T2_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T2_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2_COUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2_COUNTER` writer - "]
pub struct T2_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> T2_COUNTER_W<'a> {
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
    pub fn t2_counter(&self) -> T2_COUNTER_R {
        T2_COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t2_counter(&mut self) -> T2_COUNTER_W {
        T2_COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER2 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2_counter](index.html) module"]
pub struct T2_COUNTER_SPEC;
impl crate::RegisterSpec for T2_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t2_counter::R](R) reader structure"]
impl crate::Readable for T2_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t2_counter::W](W) writer structure"]
impl crate::Writable for T2_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T2_COUNTER to value 0"]
impl crate::Resettable for T2_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
