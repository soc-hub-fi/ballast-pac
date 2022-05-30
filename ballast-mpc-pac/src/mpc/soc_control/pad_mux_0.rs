#[doc = "Register `PAD_MUX_0` reader"]
pub struct R(crate::R<PAD_MUX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_0` writer"]
pub struct W(crate::W<PAD_MUX_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_0_SPEC>;
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
impl From<crate::W<PAD_MUX_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_MUX_0` reader - "]
pub struct PAD_MUX_0_R(crate::FieldReader<u32>);
impl PAD_MUX_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PAD_MUX_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_MUX_0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_MUX_0` writer - "]
pub struct PAD_MUX_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_MUX_0_W<'a> {
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
    pub fn pad_mux_0(&self) -> PAD_MUX_0_R {
        PAD_MUX_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pad_mux_0(&mut self) -> PAD_MUX_0_W {
        PAD_MUX_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_0](index.html) module"]
pub struct PAD_MUX_0_SPEC;
impl crate::RegisterSpec for PAD_MUX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_0::R](R) reader structure"]
impl crate::Readable for PAD_MUX_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_0::W](W) writer structure"]
impl crate::Writable for PAD_MUX_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_MUX_0 to value 0"]
impl crate::Resettable for PAD_MUX_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
