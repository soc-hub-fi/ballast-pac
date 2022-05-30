#[doc = "Register `REG_FILT_CMD` reader"]
pub struct R(crate::R<REG_FILT_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_FILT_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_FILT_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_FILT_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_FILT_CMD` writer"]
pub struct W(crate::W<REG_FILT_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_FILT_CMD_SPEC>;
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
impl From<crate::W<REG_FILT_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_FILT_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Filter start flag, write only, write 1 to start the filter :"]
pub struct START_R(crate::FieldReader<bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Filter start flag, write only, write 1 to start the filter :"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
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
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_filt_cmd](index.html) module"]
pub struct REG_FILT_CMD_SPEC;
impl crate::RegisterSpec for REG_FILT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_filt_cmd::R](R) reader structure"]
impl crate::Readable for REG_FILT_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_filt_cmd::W](W) writer structure"]
impl crate::Writable for REG_FILT_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_FILT_CMD to value 0"]
impl crate::Resettable for REG_FILT_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
