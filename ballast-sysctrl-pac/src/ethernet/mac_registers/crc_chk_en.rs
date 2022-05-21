#[doc = "Register `CRC_chk_en` reader"]
pub struct R(crate::R<CRC_CHK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CHK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CHK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CHK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_chk_en` writer"]
pub struct W(crate::W<CRC_CHK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CHK_EN_SPEC>;
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
impl From<crate::W<CRC_CHK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CHK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_chk_en` reader - By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
pub struct CRC_CHK_EN_R(crate::FieldReader<bool, bool>);
impl CRC_CHK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_CHK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_CHK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_chk_en` writer - By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
pub struct CRC_CHK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_CHK_EN_W<'a> {
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
    #[doc = "Bit 0 - By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
    #[inline(always)]
    pub fn crc_chk_en(&self) -> CRC_CHK_EN_R {
        CRC_CHK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
    #[inline(always)]
    pub fn crc_chk_en(&mut self) -> CRC_CHK_EN_W {
        CRC_CHK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_chk_en](index.html) module"]
pub struct CRC_CHK_EN_SPEC;
impl crate::RegisterSpec for CRC_CHK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_chk_en::R](R) reader structure"]
impl crate::Readable for CRC_CHK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_chk_en::W](W) writer structure"]
impl crate::Writable for CRC_CHK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC_chk_en to value 0"]
impl crate::Resettable for CRC_CHK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
