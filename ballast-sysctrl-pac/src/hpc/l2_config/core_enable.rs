#[doc = "Register `core_enable` reader"]
pub struct R(crate::R<CORE_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_enable` writer"]
pub struct W(crate::W<CORE_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_ENABLE_SPEC>;
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
impl From<crate::W<CORE_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core_enable` reader - "]
pub struct CORE_ENABLE_R(crate::FieldReader<u64, u64>);
impl CORE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        CORE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_ENABLE_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `core_enable` writer - "]
pub struct CORE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_ENABLE_W<'a> {
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
    pub fn core_enable(&self) -> CORE_ENABLE_R {
        CORE_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core_enable(&mut self) -> CORE_ENABLE_W {
        CORE_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable shadow L1 caches in L2;NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_enable](index.html) module"]
pub struct CORE_ENABLE_SPEC;
impl crate::RegisterSpec for CORE_ENABLE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core_enable::R](R) reader structure"]
impl crate::Readable for CORE_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_enable::W](W) writer structure"]
impl crate::Writable for CORE_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets core_enable to value 0"]
impl crate::Resettable for CORE_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
