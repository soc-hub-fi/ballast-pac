#[doc = "Register `MAC_rx_add_prom_wr` reader"]
pub struct R(crate::R<MAC_RX_ADD_PROM_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RX_ADD_PROM_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RX_ADD_PROM_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RX_ADD_PROM_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_rx_add_prom_wr` writer"]
pub struct W(crate::W<MAC_RX_ADD_PROM_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RX_ADD_PROM_WR_SPEC>;
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
impl From<crate::W<MAC_RX_ADD_PROM_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RX_ADD_PROM_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_rx_add_prom_wr` reader - "]
pub struct MAC_RX_ADD_PROM_WR_R(crate::FieldReader<bool, bool>);
impl MAC_RX_ADD_PROM_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAC_RX_ADD_PROM_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_RX_ADD_PROM_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_rx_add_prom_wr` writer - "]
pub struct MAC_RX_ADD_PROM_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_RX_ADD_PROM_WR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mac_rx_add_prom_wr(&self) -> MAC_RX_ADD_PROM_WR_R {
        MAC_RX_ADD_PROM_WR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mac_rx_add_prom_wr(&mut self) -> MAC_RX_ADD_PROM_WR_W {
        MAC_RX_ADD_PROM_WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rx_add_prom_wr](index.html) module"]
pub struct MAC_RX_ADD_PROM_WR_SPEC;
impl crate::RegisterSpec for MAC_RX_ADD_PROM_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rx_add_prom_wr::R](R) reader structure"]
impl crate::Readable for MAC_RX_ADD_PROM_WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rx_add_prom_wr::W](W) writer structure"]
impl crate::Writable for MAC_RX_ADD_PROM_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_rx_add_prom_wr to value 0"]
impl crate::Resettable for MAC_RX_ADD_PROM_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
