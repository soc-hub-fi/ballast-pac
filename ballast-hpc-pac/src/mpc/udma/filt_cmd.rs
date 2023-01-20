#[doc = "Register `FILT_CMD` reader"]
pub struct R(crate::R<FILT_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT_CMD` writer"]
pub struct W(crate::W<FILT_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT_CMD_SPEC>;
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
impl From<crate::W<FILT_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Filter start flag, write only, write 1 to start the filter :"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Filter start flag, write only, write 1 to start the filter :"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILT_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter start flag, write only, write 1 to start the filter :"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter start flag, write only, write 1 to start the filter :"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt_cmd](index.html) module"]
pub struct FILT_CMD_SPEC;
impl crate::RegisterSpec for FILT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filt_cmd::R](R) reader structure"]
impl crate::Readable for FILT_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt_cmd::W](W) writer structure"]
impl crate::Writable for FILT_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILT_CMD to value 0"]
impl crate::Resettable for FILT_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
