#[doc = "Register `MDIO_WrData` reader"]
pub struct R(crate::R<MDIO_WRDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_WRDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_WRDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_WRDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIO_WrData` writer"]
pub struct W(crate::W<MDIO_WRDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIO_WRDATA_SPEC>;
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
impl From<crate::W<MDIO_WRDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIO_WRDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIO_WrData` reader - "]
pub struct MDIO_WRDATA_R(crate::FieldReader<u16>);
impl MDIO_WRDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MDIO_WRDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIO_WRDATA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIO_WrData` writer - "]
pub struct MDIO_WRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIO_WRDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_wr_data(&self) -> MDIO_WRDATA_R {
        MDIO_WRDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_wr_data(&mut self) -> MDIO_WRDATA_W {
        MDIO_WRDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data to write over MDIO. Used whenever a write operation is intiated with a write to the MDIO_Ctrl register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_wr_data](index.html) module"]
pub struct MDIO_WRDATA_SPEC;
impl crate::RegisterSpec for MDIO_WRDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_wr_data::R](R) reader structure"]
impl crate::Readable for MDIO_WRDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdio_wr_data::W](W) writer structure"]
impl crate::Writable for MDIO_WRDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIO_WrData to value 0"]
impl crate::Resettable for MDIO_WRDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
