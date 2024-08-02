use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    // [For Account-Level Roles Only]
    BecomeUser,               // Users - act as
    ImportSis,                // SIS Data - import
    ManageAccountMemberships, // Admins - add / remove
    ManageAccountSettings,    // Account-level settings - manage
    ManageAlerts,             // Global announcements - add / edit / delete
    ManageCatalog,            // Catalog - manage
    // Manage Course Templates granular permissions
    AddCourseTemplate,    // Course Templates - add
    DeleteCourseTemplate, // Course Templates - delete
    EditCourseTemplate,   // Course Templates - edit
    ManageCoursesAdd,     // Courses - add
    ManageCoursesAdmin,   // Courses - manage / update
    ManageDeveloperKeys,  // Developer keys - manage
    ManageFeatureFlags,   // Feature Options - enable / disable
    ManageMasterCourses,  // Blueprint Courses - add / edit / associate / delete
    ManageRoleOverrides,  // Permissions - manage
    ManageStorageQuotas,  // Storage Quotas - manage
    ManageSis,            // SIS data - manage
    // Manage Temporary Enrollments granular permissions
    TemporaryEnrollmentsAdd,    // Temporary Enrollments - add
    TemporaryEnrollmentsEdit,   // Temporary Enrollments - edit
    TemporaryEnrollmentsDelete, // Temporary Enrollments - delete
    ManageUserLogins,           // Users - manage login details
    ManageUserObservers,        // Users - manage observers
    ModerateUserContent,        // Users - moderate content
    ReadCourseContent,          // Course Content - view
    ReadCourseList,             // Courses - view list
    ViewCourseChanges,          // Courses - view change logs
    ViewFeatureFlags,           // Feature Options - view
    ViewGradeChanges,           // Grades - view change logs
    ViewNotifications,          // Notifications - view
    ViewQuizAnswerAudits,       // Quizzes - view submission log
    ViewStatistics,             // Statistics - view
    UndeleteCourses,            // Courses - undelete

    // [For both Account-Level and Course-Level roles]
    // Note: Applicable enrollment types for course-level roles are given in brackets:
    // S = student, T = teacher (instructor), A = TA, D = designer, O = observer.
    // Lower-case letters indicate permissions that are off by default.
    // A missing letter indicates the permission cannot be enabled for the role
    // or any derived custom roles.
    AllowCourseAdminActions, // [ Tad ] Users - allow administrative actions in courses
    CreateCollaborations,    // [STADo] Student Collaborations - create
    CreateConferences,       // [STADo] Web conferences - create
    CreateForum,             // [STADo] Discussions - create
    GenerateObserverPairingCode, // [ tado] Users - Generate observer pairing codes for students
    ImportOutcomes,          // [ TaDo] Learning Outcomes - import
    LtiAddEdit,              // [ TAD ] LTI - add / edit / delete
    ManageAccountBanks,      // [ td  ] Item Banks - manage account
    ShareBanksWithSubaccounts, // [ tad ] Item Banks - share with subaccounts
    ManageAssignments,       // [ TADo] Assignments and Quizzes - add / edit / delete (deprecated)
    // Manage Assignments and Quizzes granular permissions
    ManageAssignmentsAdd,    // [ TADo] Assignments and Quizzes - add
    ManageAssignmentsEdit,   // [ TADo] Assignments and Quizzes - edit / manage
    ManageAssignmentsDelete, // [ TADo] Assignments and Quizzes - delete
    ManageCalendar,          // [sTADo] Course Calendar - add / edit / delete
    ManageContent,           // [ TADo] Course Content - add / edit / delete
    ManageCourseVisibility,  // [ TAD ] Course - change visibility
    // Manage Courses granular permissions
    ManageCoursesConclude, // [ TaD ] Courses - conclude
    ManageCoursesDelete,   // [ TaD ] Courses - delete
    ManageCoursesPublish,  // [ TaD ] Courses - publish
    ManageCoursesReset,    // [ TaD ] Courses - reset
    // Manage Files granular permissions
    ManageFilesAdd,    // [ TADo] Course Files - add
    ManageFilesEdit,   // [ TADo] Course Files - edit
    ManageFilesDelete, // [ TADo] Course Files - delete
    ManageGrades,      // [ TA  ] Grades - edit
    // Manage Groups granular permissions
    ManageGroupsAdd,               // [ TAD ] Groups - add
    ManageGroupsDelete,            // [ TAD ] Groups - delete
    ManageGroupsManage,            // [ TAD ] Groups - manage
    ManageInteractionAlerts,       // [ Ta  ] Alerts - add / edit / delete
    ManageOutcomes,                // [sTaDo] Learning Outcomes - add / edit / delete
    ManageProficiencyCalculations, // [ t d ] Outcome Proficiency Calculations - add / edit / delete
    ManageProficiencyScales, // [ t d ] Outcome Proficiency/Mastery Scales - add / edit / delete
    // Manage Sections granular permissions
    ManageSectionsAdd,    // [ TaD ] Course Sections - add
    ManageSectionsEdit,   // [ TaD ] Course Sections - edit
    ManageSectionsDelete, // [ TaD ] Course Sections - delete
    ManageStudents,       // [ TAD ] Users - manage students in courses
    ManageUserNotes,      // [ TA  ] Faculty Journal - manage entries
    ManageRubrics,        // [ TAD ] Rubrics - add / edit / delete
    // Manage Pages granular permissions
    ManageWikiCreate,   // [ TADo] Pages - create
    ManageWikiDelete,   // [ TADo] Pages - delete
    ManageWikiUpdate,   // [ TADo] Pages - update
    ModerateForum,      // [sTADo] Discussions - moderate
    PostToForum,        // [STADo] Discussions - post
    ReadAnnouncements,  // [STADO] Announcements - view
    ReadEmailAddresses, // [sTAdo] Users - view primary email address
    ReadForum,          // [STADO] Discussions - view
    ReadQuestionBanks,  // [ TADo] Question banks - view and link
    ReadReports,        // [ TAD ] Reports - manage
    ReadRoster,         // [STADo] Users - view list
    ReadSis,            // [sTa  ] SIS Data - read
    SelectFinalGrade,   // [ TA  ] Grades - select final grade for moderation
    SendMessages,       // [STADo] Conversations - send messages to individual course members
    SendMessagesAll,    // [sTADo] Conversations - send messages to entire class
    // Users - Teacher granular permissions
    AddTeacherToCourse,      // [ Tad ] Add a teacher enrollment to a course
    RemoveTeacherFromCourse, // [ Tad ] Remove a Teacher enrollment from a course
    // Users - TA granular permissions
    AddTaToCourse,      // [ Tad ] Add a TA enrollment to a course
    RemoveTaFromCourse, // [ Tad ] Remove a TA enrollment from a course
    // Users - Designer granular permissions
    AddDesignerToCourse,      // [ Tad ] Add a designer enrollment to a course
    RemoveDesignerFromCourse, // [ Tad ] Remove a designer enrollment from a course
    // Users - Observer granular permissions
    AddObserverToCourse,      // [ Tad ] Add an observer enrollment to a course
    RemoveObserverFromCourse, // [ Tad ] Remove an observer enrollment from a course
    // Users - Student granular permissions
    AddStudentToCourse,      // [ Tad ] Add a student enrollment to a course
    RemoveStudentFromCourse, // [ Tad ] Remove a student enrollment from a course
    ViewAllGrades,           // [ TAd ] Grades - view all grades
    ViewAnalytics,           // [sTA  ] Analytics - view pages
    ViewAuditTrail,          // [ t   ] Grades - view audit trail
    ViewGroupPages,          // [sTADo] Groups - view all student groups
    ViewUserLogins,          // [ TA  ] Users - view login IDs
}
