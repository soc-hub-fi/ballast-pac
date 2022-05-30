#[doc = "Register `clustercfg_addr_length` reader"]
pub struct R(crate::R<CLUSTERCFG_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUSTERCFG_ADDR_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUSTERCFG_ADDR_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUSTERCFG_ADDR_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clustercfg_addr_length` writer"]
pub struct W(crate::W<CLUSTERCFG_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUSTERCFG_ADDR_LENGTH_SPEC>;
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
impl From<crate::W<CLUSTERCFG_ADDR_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUSTERCFG_ADDR_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clustercfg_addr_length` reader - "]
pub struct CLUSTERCFG_ADDR_LENGTH_R(crate::FieldReader<u64>);
impl CLUSTERCFG_ADDR_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        CLUSTERCFG_ADDR_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLUSTERCFG_ADDR_LENGTH_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clustercfg_addr_length` writer - "]
pub struct CLUSTERCFG_ADDR_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUSTERCFG_ADDR_LENGTH_W<'a> {
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
    pub fn clustercfg_addr_length(&self) -> CLUSTERCFG_ADDR_LENGTH_R {
        CLUSTERCFG_ADDR_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn clustercfg_addr_length(&mut self) -> CLUSTERCFG_ADDR_LENGTH_W {
        CLUSTERCFG_ADDR_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cluster config address space length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clustercfg_addr_length](index.html) module"]
pub struct CLUSTERCFG_ADDR_LENGTH_SPEC;
impl crate::RegisterSpec for CLUSTERCFG_ADDR_LENGTH_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [clustercfg_addr_length::R](R) reader structure"]
impl crate::Readable for CLUSTERCFG_ADDR_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clustercfg_addr_length::W](W) writer structure"]
impl crate::Writable for CLUSTERCFG_ADDR_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clustercfg_addr_length to value 0"]
impl crate::Resettable for CLUSTERCFG_ADDR_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
