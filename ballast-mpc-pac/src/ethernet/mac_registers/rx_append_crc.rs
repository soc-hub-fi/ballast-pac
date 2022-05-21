#[doc = "Register `RX_APPEND_CRC` reader"]
pub struct R(crate::R<RX_APPEND_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_APPEND_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_APPEND_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_APPEND_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_APPEND_CRC` writer"]
pub struct W(crate::W<RX_APPEND_CRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_APPEND_CRC_SPEC>;
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
impl From<crate::W<RX_APPEND_CRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_APPEND_CRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_APPEND_CRC` reader - In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
pub struct RX_APPEND_CRC_R(crate::FieldReader<bool, bool>);
impl RX_APPEND_CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_APPEND_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_APPEND_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_APPEND_CRC` writer - In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
pub struct RX_APPEND_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_APPEND_CRC_W<'a> {
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
    #[doc = "Bit 0 - In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
    #[inline(always)]
    pub fn rx_append_crc(&self) -> RX_APPEND_CRC_R {
        RX_APPEND_CRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
    #[inline(always)]
    pub fn rx_append_crc(&mut self) -> RX_APPEND_CRC_W {
        RX_APPEND_CRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_append_crc](index.html) module"]
pub struct RX_APPEND_CRC_SPEC;
impl crate::RegisterSpec for RX_APPEND_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_append_crc::R](R) reader structure"]
impl crate::Readable for RX_APPEND_CRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_append_crc::W](W) writer structure"]
impl crate::Writable for RX_APPEND_CRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_APPEND_CRC to value 0"]
impl crate::Resettable for RX_APPEND_CRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
