#[doc = "Register `CLUSTER_BOOT_ADDR0` reader"]
pub struct R(crate::R<CLUSTER_BOOT_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUSTER_BOOT_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUSTER_BOOT_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUSTER_BOOT_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLUSTER_BOOT_ADDR0` writer"]
pub struct W(crate::W<CLUSTER_BOOT_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUSTER_BOOT_ADDR0_SPEC>;
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
impl From<crate::W<CLUSTER_BOOT_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUSTER_BOOT_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLUSTER_BOOT_ADDR0` reader - "]
pub type CLUSTER_BOOT_ADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLUSTER_BOOT_ADDR0` writer - "]
pub type CLUSTER_BOOT_ADDR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLUSTER_BOOT_ADDR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cluster_boot_addr0(&self) -> CLUSTER_BOOT_ADDR0_R {
        CLUSTER_BOOT_ADDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cluster_boot_addr0(&mut self) -> CLUSTER_BOOT_ADDR0_W<0> {
        CLUSTER_BOOT_ADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cluster_boot_addr0](index.html) module"]
pub struct CLUSTER_BOOT_ADDR0_SPEC;
impl crate::RegisterSpec for CLUSTER_BOOT_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cluster_boot_addr0::R](R) reader structure"]
impl crate::Readable for CLUSTER_BOOT_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cluster_boot_addr0::W](W) writer structure"]
impl crate::Writable for CLUSTER_BOOT_ADDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLUSTER_BOOT_ADDR0 to value 0"]
impl crate::Resettable for CLUSTER_BOOT_ADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
