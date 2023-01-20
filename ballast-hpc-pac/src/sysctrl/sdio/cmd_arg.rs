#[doc = "Register `CMD_ARG` writer"]
pub struct W(crate::W<CMD_ARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_ARG_SPEC>;
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
impl From<crate::W<CMD_ARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_ARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_ARGUMENT` writer - "]
pub type CMD_ARGUMENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_ARG_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_argument(&mut self) -> CMD_ARGUMENT_W<0> {
        CMD_ARGUMENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_arg](index.html) module"]
pub struct CMD_ARG_SPEC;
impl crate::RegisterSpec for CMD_ARG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd_arg::W](W) writer structure"]
impl crate::Writable for CMD_ARG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_ARG to value 0"]
impl crate::Resettable for CMD_ARG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
