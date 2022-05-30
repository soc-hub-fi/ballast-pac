#[doc = "Register `INTTYPE_16_31` reader"]
pub struct R(crate::R<INTTYPE_16_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE_16_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE_16_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE_16_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE_16_31` writer"]
pub struct W(crate::W<INTTYPE_16_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE_16_31_SPEC>;
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
impl From<crate::W<INTTYPE_16_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE_16_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTYPE1` reader - "]
pub struct INTTYPE1_R(crate::FieldReader<u32>);
impl INTTYPE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTTYPE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTYPE1_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTYPE1` writer - "]
pub struct INTTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE1_W<'a> {
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
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype1(&mut self) -> INTTYPE1_W {
        INTTYPE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[31:16\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2’b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b11: RFU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype_16_31](index.html) module"]
pub struct INTTYPE_16_31_SPEC;
impl crate::RegisterSpec for INTTYPE_16_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype_16_31::R](R) reader structure"]
impl crate::Readable for INTTYPE_16_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype_16_31::W](W) writer structure"]
impl crate::Writable for INTTYPE_16_31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTYPE_16_31 to value 0"]
impl crate::Resettable for INTTYPE_16_31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
