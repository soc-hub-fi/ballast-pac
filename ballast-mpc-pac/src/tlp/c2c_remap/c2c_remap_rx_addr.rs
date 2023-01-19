#[doc = "Register `c2c_remap_rx_addr` reader"]
pub struct R(crate::R<C2C_REMAP_RX_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2C_REMAP_RX_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2C_REMAP_RX_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2C_REMAP_RX_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c2c_remap_rx_addr` writer"]
pub struct W(crate::W<C2C_REMAP_RX_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2C_REMAP_RX_ADDR_SPEC>;
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
impl From<crate::W<C2C_REMAP_RX_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2C_REMAP_RX_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c2c_remap_rx_addr` reader - "]
pub type C2C_REMAP_RX_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `c2c_remap_rx_addr` writer - "]
pub type C2C_REMAP_RX_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, C2C_REMAP_RX_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c2c_remap_rx_addr(&self) -> C2C_REMAP_RX_ADDR_R {
        C2C_REMAP_RX_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_remap_rx_addr(&mut self) -> C2C_REMAP_RX_ADDR_W<0> {
        C2C_REMAP_RX_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2c_remap_rx_addr](index.html) module"]
pub struct C2C_REMAP_RX_ADDR_SPEC;
impl crate::RegisterSpec for C2C_REMAP_RX_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2c_remap_rx_addr::R](R) reader structure"]
impl crate::Readable for C2C_REMAP_RX_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2c_remap_rx_addr::W](W) writer structure"]
impl crate::Writable for C2C_REMAP_RX_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets c2c_remap_rx_addr to value 0"]
impl crate::Resettable for C2C_REMAP_RX_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
