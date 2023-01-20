#[doc = "Register `INTTYPE_48_63` reader"]
pub struct R(crate::R<INTTYPE_48_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE_48_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE_48_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE_48_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE_48_63` writer"]
pub struct W(crate::W<INTTYPE_48_63_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE_48_63_SPEC>;
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
impl From<crate::W<INTTYPE_48_63_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE_48_63_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTYPE1` reader - "]
pub type INTTYPE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTTYPE1` writer - "]
pub type INTTYPE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTTYPE_48_63_SPEC, u32, u32, 32, O>;
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
    #[must_use]
    pub fn inttype1(&mut self) -> INTTYPE1_W<0> {
        INTTYPE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[63:48\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype_48_63](index.html) module"]
pub struct INTTYPE_48_63_SPEC;
impl crate::RegisterSpec for INTTYPE_48_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype_48_63::R](R) reader structure"]
impl crate::Readable for INTTYPE_48_63_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype_48_63::W](W) writer structure"]
impl crate::Writable for INTTYPE_48_63_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTTYPE_48_63 to value 0"]
impl crate::Resettable for INTTYPE_48_63_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
