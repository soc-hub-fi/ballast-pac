#[doc = "Register `mtime` reader"]
pub struct R(crate::R<MTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mtime` writer"]
pub struct W(crate::W<MTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIME_SPEC>;
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
impl From<crate::W<MTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mtime` reader - "]
pub struct MTIME_R(crate::FieldReader<u64>);
impl MTIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        MTIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIME_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mtime` writer - "]
pub struct MTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MTIME_W<'a> {
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
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn mtime(&mut self) -> MTIME_W {
        MTIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime](index.html) module"]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtime::R](R) reader structure"]
impl crate::Readable for MTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtime::W](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
