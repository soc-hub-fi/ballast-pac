#[doc = "Register `PADCFG0` reader"]
pub struct R(crate::R<PADCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG0` writer"]
pub struct W(crate::W<PADCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG0_SPEC>;
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
impl From<crate::W<PADCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG0` reader - "]
pub struct PADCFG0_R(crate::FieldReader<u32>);
impl PADCFG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADCFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADCFG0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADCFG0` writer - "]
pub struct PADCFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PADCFG0_W<'a> {
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
    pub fn padcfg0(&self) -> PADCFG0_R {
        PADCFG0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg0(&mut self) -> PADCFG0_W {
        PADCFG0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg0](index.html) module"]
pub struct PADCFG0_SPEC;
impl crate::RegisterSpec for PADCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg0::R](R) reader structure"]
impl crate::Readable for PADCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg0::W](W) writer structure"]
impl crate::Writable for PADCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADCFG0 to value 0"]
impl crate::Resettable for PADCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
