#[doc = "Register `PADCFG3` reader"]
pub struct R(crate::R<PADCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG3` writer"]
pub struct W(crate::W<PADCFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG3_SPEC>;
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
impl From<crate::W<PADCFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG3` reader - "]
pub struct PADCFG3_R(crate::FieldReader<u32, u32>);
impl PADCFG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADCFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADCFG3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADCFG3` writer - "]
pub struct PADCFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PADCFG3_W<'a> {
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
    pub fn padcfg3(&self) -> PADCFG3_R {
        PADCFG3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg3(&mut self) -> PADCFG3_W {
        PADCFG3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg3](index.html) module"]
pub struct PADCFG3_SPEC;
impl crate::RegisterSpec for PADCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg3::R](R) reader structure"]
impl crate::Readable for PADCFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg3::W](W) writer structure"]
impl crate::Writable for PADCFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADCFG3 to value 0"]
impl crate::Resettable for PADCFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
