#[doc = "Register `INTEN_00_31` reader"]
pub struct R(crate::R<INTEN_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN_00_31` writer"]
pub struct W(crate::W<INTEN_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_00_31_SPEC>;
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
impl From<crate::W<INTEN_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - Bit 31 - 0 INTEN (R/W) GPIO\\[31:0\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
pub struct INTEN_R(crate::FieldReader<u32>);
impl INTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - Bit 31 - 0 INTEN (R/W) GPIO\\[31:0\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bit 31 - 0 INTEN (R/W) GPIO\\[31:0\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bit 31 - 0 INTEN (R/W) GPIO\\[31:0\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_00_31](index.html) module"]
pub struct INTEN_00_31_SPEC;
impl crate::RegisterSpec for INTEN_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten_00_31::R](R) reader structure"]
impl crate::Readable for INTEN_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten_00_31::W](W) writer structure"]
impl crate::Writable for INTEN_00_31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN_00_31 to value 0"]
impl crate::Resettable for INTEN_00_31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
