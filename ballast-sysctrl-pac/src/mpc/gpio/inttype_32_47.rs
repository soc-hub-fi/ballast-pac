#[doc = "Register `INTTYPE_32_47` reader"]
pub struct R(crate::R<INTTYPE_32_47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE_32_47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE_32_47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE_32_47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE_32_47` writer"]
pub struct W(crate::W<INTTYPE_32_47_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE_32_47_SPEC>;
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
impl From<crate::W<INTTYPE_32_47_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE_32_47_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTYPE0` reader - "]
pub struct INTTYPE0_R(crate::FieldReader<u32>);
impl INTTYPE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTTYPE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTYPE0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTYPE0` writer - "]
pub struct INTTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype0(&self) -> INTTYPE0_R {
        INTTYPE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype0(&mut self) -> INTTYPE0_W {
        INTTYPE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[47:32\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2???b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2???b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2???b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2???b11: RFU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype_32_47](index.html) module"]
pub struct INTTYPE_32_47_SPEC;
impl crate::RegisterSpec for INTTYPE_32_47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype_32_47::R](R) reader structure"]
impl crate::Readable for INTTYPE_32_47_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype_32_47::W](W) writer structure"]
impl crate::Writable for INTTYPE_32_47_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTYPE_32_47 to value 0"]
impl crate::Resettable for INTTYPE_32_47_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
