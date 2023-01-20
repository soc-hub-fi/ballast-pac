#[doc = "Register `MDIO_PhyAddr` reader"]
pub struct R(crate::R<MDIO_PHY_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_PHY_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_PHY_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_PHY_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIO_PhyAddr` writer"]
pub struct W(crate::W<MDIO_PHY_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIO_PHY_ADDR_SPEC>;
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
impl From<crate::W<MDIO_PHY_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIO_PHY_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIO_PhyAddr` reader - "]
pub type MDIO_PHY_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDIO_PhyAddr` writer - "]
pub type MDIO_PHY_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MDIO_PHY_ADDR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn mdio_phy_addr(&self) -> MDIO_PHY_ADDR_R {
        MDIO_PHY_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_phy_addr(&mut self) -> MDIO_PHY_ADDR_W<0> {
        MDIO_PHY_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO PHY address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_phy_addr](index.html) module"]
pub struct MDIO_PHY_ADDR_SPEC;
impl crate::RegisterSpec for MDIO_PHY_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_phy_addr::R](R) reader structure"]
impl crate::Readable for MDIO_PHY_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdio_phy_addr::W](W) writer structure"]
impl crate::Writable for MDIO_PHY_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIO_PhyAddr to value 0"]
impl crate::Resettable for MDIO_PHY_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
