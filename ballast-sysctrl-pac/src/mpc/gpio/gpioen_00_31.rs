#[doc = "Register `GPIOEN_00_31` reader"]
pub struct R(crate::R<GPIOEN_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOEN_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOEN_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOEN_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOEN_00_31` writer"]
pub struct W(crate::W<GPIOEN_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOEN_00_31_SPEC>;
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
impl From<crate::W<GPIOEN_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOEN_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOEN` reader - "]
pub struct GPIOEN_R(crate::FieldReader<u32>);
impl GPIOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOEN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOEN` writer - "]
pub struct GPIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEN_W<'a> {
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
    pub fn gpioen(&self) -> GPIOEN_R {
        GPIOEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpioen(&mut self) -> GPIOEN_W {
        GPIOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1’b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1’b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if it’s direction is configured in input mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioen_00_31](index.html) module"]
pub struct GPIOEN_00_31_SPEC;
impl crate::RegisterSpec for GPIOEN_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioen_00_31::R](R) reader structure"]
impl crate::Readable for GPIOEN_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioen_00_31::W](W) writer structure"]
impl crate::Writable for GPIOEN_00_31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOEN_00_31 to value 0"]
impl crate::Resettable for GPIOEN_00_31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
