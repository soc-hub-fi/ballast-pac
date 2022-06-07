#[doc = "Register `non_idempotent_region_addr_base1` reader"]
pub struct R(crate::R<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `non_idempotent_region_addr_base1` writer"]
pub struct W(crate::W<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>;
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
impl From<crate::W<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `non_idempotent_region_addr_base1` reader - "]
pub struct NON_IDEMPOTENT_REGION_ADDR_BASE1_R(crate::FieldReader<u64>);
impl NON_IDEMPOTENT_REGION_ADDR_BASE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        NON_IDEMPOTENT_REGION_ADDR_BASE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NON_IDEMPOTENT_REGION_ADDR_BASE1_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `non_idempotent_region_addr_base1` writer - "]
pub struct NON_IDEMPOTENT_REGION_ADDR_BASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NON_IDEMPOTENT_REGION_ADDR_BASE1_W<'a> {
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
    pub fn non_idempotent_region_addr_base1(&self) -> NON_IDEMPOTENT_REGION_ADDR_BASE1_R {
        NON_IDEMPOTENT_REGION_ADDR_BASE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn non_idempotent_region_addr_base1(&mut self) -> NON_IDEMPOTENT_REGION_ADDR_BASE1_W {
        NON_IDEMPOTENT_REGION_ADDR_BASE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [non_idempotent_region_addr_base1](index.html) module"]
pub struct NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC;
impl crate::RegisterSpec for NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [non_idempotent_region_addr_base1::R](R) reader structure"]
impl crate::Readable for NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [non_idempotent_region_addr_base1::W](W) writer structure"]
impl crate::Writable for NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets non_idempotent_region_addr_base1 to value 0"]
impl crate::Resettable for NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
