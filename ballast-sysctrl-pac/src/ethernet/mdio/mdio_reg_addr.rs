#[doc = "Register `MDIO_RegAddr` reader"]
pub struct R(crate::R<MDIO_REGADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_REGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_REGADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_REGADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIO_RegAddr` writer"]
pub struct W(crate::W<MDIO_REGADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIO_REGADDR_SPEC>;
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
impl From<crate::W<MDIO_REGADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIO_REGADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIO_RegAddr` reader - "]
pub struct MDIO_REGADDR_R(crate::FieldReader<u8>);
impl MDIO_REGADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MDIO_REGADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIO_REGADDR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIO_RegAddr` writer - "]
pub struct MDIO_REGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIO_REGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn mdio_reg_addr(&self) -> MDIO_REGADDR_R {
        MDIO_REGADDR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn mdio_reg_addr(&mut self) -> MDIO_REGADDR_W {
        MDIO_REGADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO register address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_reg_addr](index.html) module"]
pub struct MDIO_REGADDR_SPEC;
impl crate::RegisterSpec for MDIO_REGADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_reg_addr::R](R) reader structure"]
impl crate::Readable for MDIO_REGADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdio_reg_addr::W](W) writer structure"]
impl crate::Writable for MDIO_REGADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIO_RegAddr to value 0"]
impl crate::Resettable for MDIO_REGADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
