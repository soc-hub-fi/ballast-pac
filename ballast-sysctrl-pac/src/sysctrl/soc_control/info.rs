#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFO` writer"]
pub struct W(crate::W<INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFO_SPEC>;
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
impl From<crate::W<INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cluster_count` reader - Number of Clusters"]
pub type CLUSTER_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cluster_count` writer - Number of Clusters"]
pub type CLUSTER_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INFO_SPEC, u16, u16, 16, O>;
#[doc = "Field `core_count` reader - Number of Cores"]
pub type CORE_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `core_count` writer - Number of Cores"]
pub type CORE_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INFO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of Clusters"]
    #[inline(always)]
    pub fn cluster_count(&self) -> CLUSTER_COUNT_R {
        CLUSTER_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Cores"]
    #[inline(always)]
    pub fn core_count(&self) -> CORE_COUNT_R {
        CORE_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Clusters"]
    #[inline(always)]
    #[must_use]
    pub fn cluster_count(&mut self) -> CLUSTER_COUNT_W<0> {
        CLUSTER_COUNT_W::new(self)
    }
    #[doc = "Bits 16:31 - Number of Cores"]
    #[inline(always)]
    #[must_use]
    pub fn core_count(&mut self) -> CORE_COUNT_W<16> {
        CORE_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [info::W](W) writer structure"]
impl crate::Writable for INFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INFO to value 0x0004_0000"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0000;
}
