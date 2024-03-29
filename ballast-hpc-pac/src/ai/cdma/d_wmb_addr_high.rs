#[doc = "Register `D_WMB_ADDR_HIGH` reader"]
pub struct R(crate::R<D_WMB_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WMB_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WMB_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WMB_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WMB_ADDR_HIGH` writer"]
pub struct W(crate::W<D_WMB_ADDR_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WMB_ADDR_HIGH_SPEC>;
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
impl From<crate::W<D_WMB_ADDR_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WMB_ADDR_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMB_ADDR_HIGH` reader - "]
pub type WMB_ADDR_HIGH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WMB_ADDR_HIGH` writer - "]
pub type WMB_ADDR_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_WMB_ADDR_HIGH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wmb_addr_high(&self) -> WMB_ADDR_HIGH_R {
        WMB_ADDR_HIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn wmb_addr_high(&mut self) -> WMB_ADDR_HIGH_W<0> {
        WMB_ADDR_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Higher 32bits of wmb address when axi araddr is 64bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_wmb_addr_high](index.html) module"]
pub struct D_WMB_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for D_WMB_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_wmb_addr_high::R](R) reader structure"]
impl crate::Readable for D_WMB_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_wmb_addr_high::W](W) writer structure"]
impl crate::Writable for D_WMB_ADDR_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_WMB_ADDR_HIGH to value 0"]
impl crate::Resettable for D_WMB_ADDR_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
