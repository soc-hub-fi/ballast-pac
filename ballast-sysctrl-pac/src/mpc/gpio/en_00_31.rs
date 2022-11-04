#[doc = "Register `EN_00_31` reader"]
pub struct R(crate::R<EN_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_00_31` writer"]
pub struct W(crate::W<EN_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_00_31_SPEC>;
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
impl From<crate::W<EN_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - "]
pub struct EN_R(crate::FieldReader<u32>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - "]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_00_31](index.html) module"]
pub struct EN_00_31_SPEC;
impl crate::RegisterSpec for EN_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_00_31::R](R) reader structure"]
impl crate::Readable for EN_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_00_31::W](W) writer structure"]
impl crate::Writable for EN_00_31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_00_31 to value 0"]
impl crate::Resettable for EN_00_31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
