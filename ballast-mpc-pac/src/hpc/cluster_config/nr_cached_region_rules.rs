#[doc = "Register `nr_cached_region_rules` reader"]
pub struct R(crate::R<NR_CACHED_REGION_RULES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NR_CACHED_REGION_RULES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NR_CACHED_REGION_RULES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NR_CACHED_REGION_RULES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `nr_cached_region_rules` writer"]
pub struct W(crate::W<NR_CACHED_REGION_RULES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NR_CACHED_REGION_RULES_SPEC>;
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
impl From<crate::W<NR_CACHED_REGION_RULES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NR_CACHED_REGION_RULES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nr_cached_region_rules` reader - "]
pub type NR_CACHED_REGION_RULES_R = crate::FieldReader<u64, u64>;
#[doc = "Field `nr_cached_region_rules` writer - "]
pub type NR_CACHED_REGION_RULES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, NR_CACHED_REGION_RULES_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn nr_cached_region_rules(&self) -> NR_CACHED_REGION_RULES_R {
        NR_CACHED_REGION_RULES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn nr_cached_region_rules(&mut self) -> NR_CACHED_REGION_RULES_W<0> {
        NR_CACHED_REGION_RULES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of cached regions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nr_cached_region_rules](index.html) module"]
pub struct NR_CACHED_REGION_RULES_SPEC;
impl crate::RegisterSpec for NR_CACHED_REGION_RULES_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [nr_cached_region_rules::R](R) reader structure"]
impl crate::Readable for NR_CACHED_REGION_RULES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nr_cached_region_rules::W](W) writer structure"]
impl crate::Writable for NR_CACHED_REGION_RULES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets nr_cached_region_rules to value 0"]
impl crate::Resettable for NR_CACHED_REGION_RULES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
